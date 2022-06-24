use std::str::FromStr;
use std::{convert::TryInto, sync::Arc};

use anyhow::Context;
use futures_util::stream::FuturesUnordered;
use num_traits::{Inv, ToPrimitive};
use serde::Serialize;
use themelio_nodeprot::{ValClient, ValClientSnapshot};
use themelio_stf::{melvm::covenant_weight_from_bytes, PoolKey};

use smol::{lock::Semaphore, prelude::*};
use themelio_structs::{Block, BlockHeight, CoinID, Denom, Header, TxHash};
use tide::Body;
use tmelcrypt::HashVal;

use crate::html::AsPoolDataItem;
use crate::utils::*;
use async_trait::async_trait;
use crate::{notfound, to_badgateway, to_badreq, State};

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct PoolInfoKey(PoolKey, BlockHeight);

#[derive(serde::Serialize)]
// A block summary for the homepage.
pub struct BlockSummary {
    pub header: Header,
    pub total_weight: u128,
    pub reward_amount: u128,
    pub transactions: Vec<TransactionSummary>,
}

#[derive(serde::Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
// A transaction summary for the homepage.
pub struct TransactionSummary {
    pub hash: String,
    pub shorthash: String,
    pub height: u64,
    pub weight: u128,
    pub mel_moved: u128,
}


#[async_trait]
trait ExtendedClient {
    async fn get_snapshot(&self) -> anyhow::Result<ValClientSnapshot>;
    async fn older_snapshot(&self, height: u64) -> anyhow::Result<ValClientSnapshot>;
    async fn get_history(&self, height: u64) -> anyhow::Result<Header>;
}   

#[async_trait]
trait ExtendedRequest {
    fn client(&self) -> ValClient;
    async fn requested_snapshot(&self) -> tide::Result<ValClientSnapshot>;
    fn height(&self) -> tide::Result<u64>;
    fn parse<T>(&self, param: &str) -> Result<T, T::Err>
        where T: FromStr;
}
#[async_trait]
impl ExtendedClient for ValClient {
    async fn get_snapshot(&self) -> anyhow::Result<ValClientSnapshot>{
        let last_snap = self
            .snapshot()
            .await
            .expect("Failed to get most recent snapshot");

        Ok(last_snap)
    }
    async fn older_snapshot(&self, height: u64) -> anyhow::Result<ValClientSnapshot>{
        let older = self.get_snapshot()
        .await?
        .get_older(height.into())
        .await
        .expect(&format!("Unable to get block at height: {height}"));

        Ok(older)
    }
    async fn get_history(&self, height: u64) -> anyhow::Result<Header>{
        let snapshot = self.get_snapshot().await?;
        let hist = snapshot.get_history(height.into())
        .await?
        .context(format!("Unable to get history at height {height}"));
        hist
    }

}



#[async_trait]
impl ExtendedRequest for tide::Request<State> {
    async fn requested_snapshot(&self) -> tide::Result<ValClientSnapshot>{
        let height = self.height()?;
        self.client().older_snapshot(height).await.map_err(to_badreq)

    }
    fn height(&self) -> tide::Result<u64> {
        let p = self.param("height")?;
        p.parse().map_err(to_badreq)
    } 
    fn parse<T>(&self, param: &str) -> Result<T, T::Err>
        where T: FromStr {
        let p = self.param(param).expect(&format!("Unable to parse param: `{param}`"));
        p.parse::<T>()

    }
    fn client(&self) -> ValClient {
        self.state().val_client
    }
}

/// Get the overview, which includes basically all the information that the homepage needs
#[tracing::instrument(skip(req))]
pub async fn get_overview(req: tide::Request<State>) -> tide::Result<Body> {
    #[derive(Serialize)]
    struct Overview {
        erg_per_mel: f64,
        sym_per_mel: f64,
        recent_blocks: Vec<BlockSummary>,
    }

    let last_snap = req.client().get_snapshot().await?;
    let mut blocks = Vec::new();
    let mut futs = get_old_blocks(&last_snap, 50);

    while let Some(inner) = futs.next().await {
        let (block, reward) = inner.map_err(to_badgateway)?;
        let transactions: Vec<TransactionSummary> = get_transactions(&block);
        blocks.push(BlockSummary {
            header: block.header,
            total_weight: block
                .transactions
                .iter()
                .map(|v| v.weight(covenant_weight_from_bytes))
                .sum(),
            reward_amount: reward.into(),
            transactions: transactions.clone(),
        });
    }

    let erg_per_mel = last_snap
        .get_pool(PoolKey::new(Denom::Mel, Denom::Erg))
        .await
        .map_err(to_badgateway)?
        .unwrap()
        .implied_price()
        .to_f64()
        .unwrap_or_default();
    let sym_per_mel = last_snap
        .get_pool(PoolKey::new(Denom::Mel, Denom::Sym))
        .await
        .map_err(to_badgateway)?
        .unwrap()
        .implied_price()
        .inv()
        .to_f64()
        .unwrap_or_default();

    Body::from_json(&Overview {
        erg_per_mel,
        sym_per_mel,
        recent_blocks: blocks,
    })
}

fn get_transactions(block: &Block) -> Vec<TransactionSummary> {
    let mut transactions: Vec<TransactionSummary> = Vec::new();
    for transaction in &block.transactions {
        transactions.push(TransactionSummary {
            hash: hex::encode(&transaction.hash_nosigs().0),
            shorthash: hex::encode(&transaction.hash_nosigs().0[0..5]),
            height: block.header.height.0,
            weight: transaction.weight(covenant_weight_from_bytes),
            mel_moved: transaction
                .outputs
                .iter()
                .map(|v| if v.denom == Denom::Mel { v.value.0 } else { 0 })
                .sum::<u128>()
                + transaction.fee.0,
        })
    }
    transactions.sort_unstable();
    transactions
}

/// Get the latest status
#[tracing::instrument(skip(req))]
pub async fn get_latest(req: tide::Request<State>) -> tide::Result<Body> {
    let last_snap = req.client().get_snapshot().await?;
    Body::from_json(&last_snap.current_header())
}

/// Get a particular block header
#[tracing::instrument(skip(req))]
pub async fn get_header(req: tide::Request<State>) -> tide::Result<Body> {
    let height = req.height()?;
    let older = req.client().get_history(height).await?;
    Body::from_json(&older)
}

/// Get a particular transaction
#[tracing::instrument(skip(req))]
pub async fn get_transaction(req: tide::Request<State>) -> tide::Result<Body> {
    let height: u64 = req.parse("height")?;
    let txhash: String = req.parse("txhash")?;
    let txhash: Vec<u8> = hex::decode(&txhash)?;
    let txhash: TxHash = HashVal(
        txhash
            .try_into()
            .map_err(|_| anyhow::anyhow!("not the right length"))
            .map_err(to_badreq)?,
    )
    .into();
    let last_snap = req.state().val_client.snapshot().await?;
    let older = last_snap.get_older(height.into()).await?;
    let tx = older.get_transaction(txhash).await?;
    Body::from_json(&tx.ok_or_else(notfound)?)
}

/// Get a particular coin
#[tracing::instrument(skip(req))]
pub async fn get_coin(req: tide::Request<State>) -> tide::Result<Body> {
    let height: u64 = req.param("height")?.parse()?;
    let coinid_string: String = req.param("coinid")?.into();
    let coinid_exploded: Vec<&str> = coinid_string.split('-').collect();
    if coinid_exploded.len() != 2 {
        return Err(to_badreq(anyhow::anyhow!("bad coinid")));
    }
    let txhash: Vec<u8> = hex::decode(&coinid_exploded[0])?;
    let txhash: TxHash = HashVal(
        txhash
            .try_into()
            .map_err(|_| anyhow::anyhow!("not the right length"))
            .map_err(to_badreq)?,
    )
    .into();
    let index: u8 = coinid_exploded[1].parse().map_err(to_badreq)?;
    let last_snap = req.state().val_client.snapshot().await?;
    let older = last_snap.get_older(height.into()).await?;
    let cdh = older.get_coin(CoinID { txhash, index }).await?;
    Body::from_json(&cdh.ok_or_else(notfound)?)
}

/// Get a particular pool
#[tracing::instrument(skip(req))]
pub async fn get_pool(req: tide::Request<State>) -> tide::Result<Body> {
    let height: u64 = req.param("height")?.parse()?;
    let denom_string: String = req.param("denom")?.into();
    let denom =
        Denom::from_bytes(&hex::decode(&denom_string).map_err(to_badreq)?).context("oh no")?;

    let last_snap = req.state().val_client.snapshot().await?;
    let older = last_snap.get_older(height.into()).await?;
    let cdh = older
        .get_pool(PoolKey::mel_and(denom))
        .await
        .map_err(to_badgateway)?;
    Body::from_json(&cdh.ok_or_else(notfound)?)
}

/// Get a particular block
#[tracing::instrument(skip(req))]
pub async fn get_full_block(req: tide::Request<State>) -> tide::Result<Body> {
    let height: u64 = req.param("height")?.parse().map_err(to_badreq)?;
    let last_snap = req
        .state()
        .val_client
        .snapshot()
        .await
        .map_err(to_badgateway)?;
    let older = last_snap
        .get_older(height.into())
        .await
        .map_err(to_badgateway)?;
    let block = older.current_block().await.map_err(to_badgateway)?;
    Body::from_json(&block)
}

/// Get block summary
#[tracing::instrument(skip(req))]
pub async fn get_block_summary(req: tide::Request<State>) -> tide::Result<Body> {
    let height: u64 = req.param("height")?.parse().map_err(to_badreq)?;
    let last_snap = req
        .state()
        .val_client
        .snapshot()
        .await
        .map_err(to_badgateway)?;
    let older = last_snap
        .get_older(height.into())
        .await
        .map_err(to_badgateway)?;
    let block = older.current_block().await.map_err(to_badgateway)?;

    let reward_coin = older
        .get_coin(CoinID::proposer_reward(height.into()))
        .await
        .map_err(to_badgateway)?;

    let reward_amount = reward_coin.map(|v| v.coin_data.value).unwrap_or_default();

    let transactions = get_transactions(&block);

    Body::from_json(&BlockSummary {
        header: block.header,
        total_weight: block
            .transactions
            .iter()
            .map(|v| v.weight(covenant_weight_from_bytes))
            .sum(),
        reward_amount: reward_amount.into(),
        transactions,
    })
}

pub async fn get_pooldata_range(req: tide::Request<State>) -> tide::Result<Body> {
    let state = req.state();
    let client = &state.val_client;
    let cache = &state.raw_pooldata_cache;
    let lower_block: u64 = req.param("lowerblock")?.parse().map_err(to_badreq)?;
    let upper_block: u64 = req.param("upperblock")?.parse().map_err(to_badreq)?;

    let pool_key = {
        let denom = req.param("denom_left").map(|v| v.to_string())?;
        let left = Denom::from_str(&denom).map_err(to_badreq)?;
        let denom = req.param("denom_right").map(|v| v.to_string())?;
        let right = Denom::from_str(&denom).map_err(to_badreq)?;
        PoolKey { left, right }
    };
    let snapshot = &client.snapshot().await.map_err(to_badgateway)?;

    let blockheight_interval = {
        if lower_block == upper_block {
            vec![lower_block]
        } else {
            interpolate_between(lower_block, upper_block, 1000)
                .filter(|x| *x > 0)
                .collect()
        }
    };

    // let mut missing: Vec<BlockHeight> = Vec::new();
    // for height in &blockheight_interval {
    //     let cache_key = PoolInfoKey(pool_key, BlockHeight(*height));
    //     if !cache.contains_key(&cache_key) {
    //         missing.push((*height).into())
    //     } else {
    //         log::debug!("cache hit {}", height);
    //     }
    // }

    let semaphore = Arc::new(Semaphore::new(128));
    let mut item_futs = FuturesUnordered::new();
    for height in &blockheight_interval {
        let semaphore = semaphore.clone();
        item_futs.push(async move {
            let _guard = semaphore.acquire().await;
            let cache_key = PoolInfoKey(pool_key, BlockHeight(*height));
            if cache.contains_key(&cache_key) {
                log::debug!("cache hit {}", height);
                let item = cache.get(&cache_key).unwrap().value().clone();
                Ok(item)
            } else {
                log::debug!("cache miss {} ({})", height, cache.len());
                let item = snapshot.get_older_pool_data_item(pool_key, *height).await?;
                cache.insert(cache_key, item.clone());
                drop(_guard);
                tide::Result::Ok(item)
            }
        });
    }
    // Gather the stuff
    let mut output = vec![];
    while let Some(res) = item_futs.next().await {
        log::debug!(
            "loading pooldata {}/{}",
            output.len() + 1,
            blockheight_interval.len()
        );
        if let Some(res) = res? {
            output.push(res);
        }
    }
    output.sort_unstable_by_key(|v| v.block_height());

    Body::from_json(&output)
}

// pub async fn get_pooldata(req: tide::Request<State>) -> tide::Result<Body> {
//     let client = req.state();
//     let lower_block: u64 = req.param("lowerblock")?.parse().map_err(to_badgateway)?;
//     let denom = Denom::from_bytes(&hex::decode("73").map_err(to_badreq)?)
//         .ok_or_else(|| to_badreq(anyhow::anyhow!("bad")))?;

//     let snapshot = client.snapshot().await?
//     .get_older(lower_block.into()).await.map_err(to_badgateway)?;
//     Body::from_json(&pool_item(&snapshot, denom).await?)
// }
