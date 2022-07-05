
use std::time::{UNIX_EPOCH, SystemTime};
use std::{convert::TryInto, sync::Arc};

use anyhow::{Context};
use dashmap::DashMap;
use futures_util::stream::FuturesUnordered;
use num_traits::{ToPrimitive};
use serde::{Serialize, Deserialize};
use themelio_nodeprot::{ValClient, ValClientSnapshot};

use smol::{lock::Semaphore, prelude::*};
use themelio_structs::{Block, BlockHeight, CoinID, Denom, TxHash, Transaction, CoinDataHeight, PoolState, MICRO_CONVERTER, PoolKey};
use tmelcrypt::HashVal;

use crate::utils::*;

use async_trait::async_trait;





#[derive(PartialEq, Eq, Hash, Clone)]

#[derive(Serialize, Deserialize)]
pub struct PoolInfoKey(PoolKey, BlockHeight);
#[derive(serde::Serialize, rweb::Schema)]
pub struct Header(pub themelio_structs::Header);

#[derive(serde::Serialize, rweb::Schema)]
// A block summary for the homepage.
pub struct BlockSummary {
    pub header: Header,
    pub total_weight: u128,
    pub reward_amount: u128,
    pub transactions: Vec<TransactionSummary>,
}

#[derive(serde::Serialize, Clone, PartialEq, Eq, PartialOrd, Ord,  rweb::Schema)]
// A transaction summary for the homepage.
pub struct TransactionSummary {
    pub hash: String,
    pub shorthash: String,
    pub height: u64,
    pub weight: u128,
    pub mel_moved: u128,
}

#[derive(Serialize, rweb::Schema)]
pub struct Overview {
    erg_per_mel: f64,
    sym_per_mel: f64,
    recent_blocks: Vec<BlockSummary>,
}


// 2 million cached pooldataitems is 64 mb
// 1 item is 256 bits
#[derive(Serialize, Deserialize, Clone)]
pub struct PoolDataItem {
    date: u64,
    height: u64,
    price: f64,
    liquidity: f64,
    ergs_per_mel: f64,
}

#[async_trait]
pub trait AsPoolDataItem {
    async fn as_pool_data_item(&self, pool_key: PoolKey) -> anyhow::Result<Option<PoolDataItem>>;
    async fn get_older_pool_data_item(
        &self,
        pool_key: PoolKey,
        height: u64,
    ) -> anyhow::Result<Option<PoolDataItem>>;
}

#[async_trait]
impl AsPoolDataItem for ValClientSnapshot {
    // returns a PoolDataItem that assumes the snapshot represents the most recent block
    async fn as_pool_data_item(&self, pool_key: PoolKey) -> anyhow::Result<Option<PoolDataItem>> {
        let height = self.current_header().height.0;
        Ok(self.get_pool(pool_key).await?.map(|pool_info| {
            let price = pool_info.implied_price().to_f64().unwrap_or_default();
            let liquidity =
                (pool_info.lefts as f64 * pool_info.rights as f64).sqrt() / MICRO_CONVERTER as f64;
            PoolDataItem {
                date: PoolDataItem::block_time(0),
                height,
                price,
                liquidity,
                ergs_per_mel: themelio_stf::dosc_to_erg(BlockHeight(height), 10000) as f64 / 10000.0,
            }
        }))
    }
    async fn get_older_pool_data_item(
        &self,
        pool_key: PoolKey,
        height: u64,
    ) -> anyhow::Result<Option<PoolDataItem>> {
        let last_height = self.current_header().height.0;
        let snapshot = self.get_older(height.into()).await?;
        let item = snapshot.as_pool_data_item(pool_key).await?;
        Ok(item.map(|mut item| item.set_time(last_height - height).clone()))
    }
}

impl PoolDataItem {
    pub fn block_time(distance_from_now: u64) -> u64 {
        let now_unix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            / 30
            * 30;
        now_unix - distance_from_now * 30
    }
    pub fn set_time(&mut self, distance_from_now: u64) -> &mut Self {
        self.date = PoolDataItem::block_time(distance_from_now);
        self
    }
    pub fn block_height(&self) -> u64 {
        self.height
    }
}


async fn get_exchange(
    last_snap: &ValClientSnapshot,
    denom1: Denom,
    denom2: Denom,
) -> anyhow::Result<f64> {
    let pool = last_snap
        .get_pool(PoolKey::new(denom1, denom2))
        .await
        .context(format!("Unable to get exchange for {denom1}/{denom2}"))?.unwrap();
    let micro = pool
    .implied_price()
    .to_f64()
    .unwrap_or_default();
    Ok(micro)
}





pub async fn get_overview(client: ValClient, height: Option<u64>) -> anyhow::Result<Overview> {

    let last_snap = match height{
        Some(height) => client.older_snapshot(height).await?,   
        None => client.snapshot().await?
    };

    let mut futs = get_old_blocks(&last_snap, 50);

    let mut blocks: Vec<BlockSummary> = vec![];
    while let Some(inner) = futs.next().await {
        let (block, reward) = inner?;
        blocks.push(create_block_summary(block, reward))
    }

    let erg_per_mel = get_exchange(&last_snap, Denom::Mel, Denom::Erg).await?;
    let sym_per_mel = get_exchange(&last_snap, Denom::Mel, Denom::Sym).await?;

    Ok(Overview {
        erg_per_mel,
        sym_per_mel,
        recent_blocks: blocks,
    })
}



pub async fn get_latest(client: ValClient) -> anyhow::Result<Header> {
    let last_snap = client.snapshot().await?;
    anyhow::Ok(Header(last_snap.current_header()))
}

pub async fn get_transaction(client: ValClient, height: u64, txhash: String) -> anyhow::Result<Transaction>{
    let txhash: Vec<u8> = hex::decode(&txhash)?;
    let txhash: TxHash = HashVal(
        txhash
            .try_into()
            .map_err(|_| anyhow::anyhow!("not the right length"))?,
    )
    .into();
    let last_snap = client.snapshot().await?;
    let older = last_snap.get_older(height.into()).await?;
    let tx = older.get_transaction(txhash).await?;
    tx.ok_or(anyhow::format_err!("TODO"))
}
pub async fn get_coin(client: ValClient, height: u64, coinid_string: String) -> anyhow::Result<CoinDataHeight> {
    let coinid_exploded: Vec<&str> = coinid_string.split('-').collect();
    if coinid_exploded.len() != 2 {
        return Err(anyhow::format_err!("bad coinid"));
    }
    let txhash: Vec<u8> = hex::decode(&coinid_exploded[0])?;
    let txhash: TxHash = HashVal(
        txhash
            .try_into()
            .map_err(|_| anyhow::anyhow!("not the right length"))?,
    )
    .into();
    let index: u8 = coinid_exploded[1].parse()?;
    let older = client.older_snapshot(height.into()).await?;
    let cdh = older.get_coin(CoinID { txhash, index }).await?;
    cdh.ok_or(anyhow::format_err!("TODO"))
}

pub async fn get_pool(client: ValClient, height: u64, denom: Denom) -> anyhow::Result<PoolState> {
    let older = client.older_snapshot(height).await?;
    let cdh = older
        .get_pool(PoolKey::mel_and(denom))
        .await?;
    cdh.ok_or(anyhow::format_err!("TODO"))
}


/// Get a particular block
// #[tracing::instrument(skip(req))]
pub async fn get_full_block(client: ValClient, height: u64) -> anyhow::Result<Block> {
    let older = client.older_snapshot(height).await?;
    let block = older.current_block().await?;
    Ok(block)
}

/// Get block summary
// #[tracing::instrument(skip(req))]
pub async fn get_block_summary(client: ValClient, height: u64) -> anyhow::Result<BlockSummary> {
    let older = client.older_snapshot(height).await?;
    let block = older.current_block().await?;
    let reward_amount = client.get_reward_amount(height).await?;
    Ok(create_block_summary(block, reward_amount))
}


pub async fn get_pooldata_range(client: ValClient,cache: &Arc<themelio_nodeprot::cache::AsyncCache>, left: Denom, right: Denom, upper_block: u64, lower_block: u64) -> anyhow::Result<Vec<PoolDataItem>> {


    let pool_key = 
        PoolKey { left, right };
    let snapshot = &client.snapshot().await?;

    let blockheight_interval = {
        if lower_block == upper_block {
            vec![lower_block]
        } else {
            interpolate_between(lower_block, upper_block, 1000)
                .filter(|x| *x > 0)
                .collect()
        }
    };


    let semaphore = Arc::new(Semaphore::new(128));
    let mut item_futs = FuturesUnordered::new();
    for height in &blockheight_interval {
        let semaphore = semaphore.clone();
        item_futs.push(async move {
            let _guard = semaphore.acquire().await;
            let cache_key = PoolInfoKey(pool_key, BlockHeight(*height));
            // let item = {
            //     if cache.contains_key(&cache_key) {
            //         log::debug!("cache hit {}", height);
            //         let item = cache.get(&cache_key).unwrap().value().clone();
            //         item
            //     } else {
            //         log::debug!("cache miss {} ({})", height, cache.len());
            //         let item = snapshot.get_older_pool_data_item(pool_key, *height).await?;
            //         cache.insert(cache_key, item.clone());
            //         drop(_guard);
            //         item

            //     }
            // };
            cache.get_or_try_fill(&cache_key, async {
                snapshot.get_older_pool_data_item(pool_key, *height)
                    .await
                    .map_err(|err| anyhow::format_err!("Failed to get snapshot"))
            }).await
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

   Ok(output)
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
