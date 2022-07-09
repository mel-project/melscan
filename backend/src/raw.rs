use std::time::{SystemTime, UNIX_EPOCH};
use std::{convert::TryInto, sync::Arc};

use anyhow::Context;
use futures_util::stream::FuturesUnordered;
use num_traits::ToPrimitive;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use smol::{lock::Semaphore, prelude::*};
use themelio_nodeprot::cache::AsyncCache;
use themelio_nodeprot::{ValClient, ValClientSnapshot};
use themelio_stf::melvm::covenant_weight_from_bytes;
use themelio_structs::{
    Block, BlockHeight, CoinDataHeight, CoinID, CoinValue, Denom, Header, PoolKey, PoolState,
    Transaction, TxHash, MICRO_CONVERTER,
};
use tmelcrypt::HashVal;

use crate::utils::*;

use async_trait::async_trait;

#[derive(PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct PoolInfoKey(PoolKey, BlockHeight);

#[derive(serde::Serialize, Debug, Clone)]
// A block summary for the homepage.
pub struct BlockSummary {
    pub header: Header,
    pub total_weight: u128,
    pub reward_amount: CoinValue,
    pub transactions: Vec<TransactionSummary>,
    pub header_hash: HashVal,
    pub total_fees: u128,
    pub fee_multiplier: f64
}

impl BlockSummary {
    /// Creates a new block summary from a full block and the reward amount
    pub fn from_block(block: Block, reward_amount: CoinValue) -> Self {
        
        let transactions: Vec<TransactionSummary> = get_transactions(&block);
        let header = block.header;
        let fee_multiplier = header.fee_multiplier as f64 / 65536.0;
        Self {
            header,
            total_weight: block
                .transactions
                .iter()
                .map(|v| v.weight(covenant_weight_from_bytes))
                .sum(),
            reward_amount,
            transactions,
            header_hash: header.hash(),
            total_fees: block.transactions.iter().map(|v| v.fee.0).sum(),
            fee_multiplier,
        }
    }
}

#[derive(serde::Serialize, Clone, PartialEq, Eq, PartialOrd, Ord, rweb::Schema, Debug)]
// A transaction summary for the homepage.
pub struct TransactionSummary {
    pub hash: String,
    pub shorthash: String,
    pub height: u64,
    pub weight: u128,
    pub mel_moved: u128,
}

#[derive(Serialize, Debug, Clone)]
pub struct Overview {
    pub erg_per_mel: f64,
    pub sym_per_mel: f64,
    pub recent_blocks: Vec<BlockSummary>,
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
#[derive(Serialize, Deserialize, Clone)]
pub struct PoolInfo {
    pool_state: PoolState,
    latest_item: PoolDataItem,
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
                ergs_per_mel: themelio_stf::dosc_to_erg(BlockHeight(height), 10000) as f64
                    / 10000.0,
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
        .context(format!("Unable to get exchange for {denom1}/{denom2}"))?
        .unwrap();
    let micro = pool.implied_price().to_f64().unwrap_or_default();
    Ok(micro)
}


#[tracing::instrument(skip(client))]
/// Generates an Overview structure from a client and height.
pub async fn get_overview(client: ValClient, height: Option<u64>) -> anyhow::Result<Overview> {
    let last_snap = match height {
        Some(height) => client.older_snapshot(height).await?,
        None => client.snapshot().await?,
    };

    let mut futs = get_old_blocks(&last_snap, 50);

    let mut blocks: Vec<BlockSummary> = vec![];
    while let Some(inner) = futs.next().await {
        let (block, reward) = inner?;
        blocks.push(BlockSummary::from_block(block, reward))
    }

    let erg_per_mel = get_exchange(&last_snap, Denom::Mel, Denom::Erg).await?;
    let sym_per_mel = get_exchange(&last_snap, Denom::Mel, Denom::Sym).await?;

    Ok(Overview {
        erg_per_mel,
        sym_per_mel,
        recent_blocks: blocks,
    })
}

/// Gets the latest blockchain header.
pub async fn get_latest_header(client: ValClient) -> anyhow::Result<Header> {
    let last_snap = client.snapshot().await?;
    Ok(last_snap.current_header())
}

/// Gets a particular transaction at a height
pub async fn get_transaction(
    client: ValClient,
    height: u64,
    txhash: String,
) -> anyhow::Result<Option<Transaction>> {
    let txhash: TxHash = TxHash(txhash.parse()?);
    let last_snap = client.snapshot().await?;
    let older = last_snap.get_older(height.into()).await?;
    Ok(older.get_transaction(txhash).await?)
}

/// Gets a particular coin at a height
pub async fn get_coin(
    client: ValClient,
    height: u64,
    coinid_string: String,
) -> anyhow::Result<Option<CoinDataHeight>> {
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
    let older = client.older_snapshot(height).await?;
    Ok(older.get_coin(CoinID { txhash, index }).await?)
}

/// Get a particular block
pub async fn get_full_block(client: ValClient, height: u64) -> anyhow::Result<Block> {
    let older = client.older_snapshot(height).await?;
    let block = older.current_block().await?;
    Ok(block)
}

/// Get block summary
pub async fn get_block_summary(client: ValClient, height: u64) -> anyhow::Result<BlockSummary> {
    let older = client.older_snapshot(height).await?;
    let block = older.current_block().await?;
    let reward_amount = client.get_reward_amount(height).await?;
    Ok(BlockSummary::from_block(block, reward_amount))
}

pub async fn get_pool(
    client: ValClient,
    height: u64,
    left: Denom,
    right: Denom,
) -> anyhow::Result<Option<PoolInfo>> {
    let older = client.older_snapshot(height).await?;
    let key = PoolKey { left, right };
    let pool_state = older
        .get_pool(key)
        .await?
        .ok_or(anyhow::format_err!("Unable to get pool state"))?;
    let latest_item = older
        .as_pool_data_item(key)
        .await?
        .ok_or(anyhow::format_err!("Unable to get pool data item"))?;
    Ok(Some(PoolInfo {
        pool_state,
        latest_item,
    }))
}

/// Obtains some pooldatas for the given range
pub async fn get_pooldata_range(
    client: ValClient,
    left: Denom,
    right: Denom,
    lower_block: u64,
    upper_block: u64,
) -> anyhow::Result<Vec<PoolDataItem>> {
    static CACHE: Lazy<AsyncCache> = Lazy::new(|| AsyncCache::new(1_000_000));

    let pool_key = PoolKey { left, right };
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
            CACHE
                .get_or_try_fill(&cache_key, async {
                    snapshot
                        .get_older_pool_data_item(pool_key, *height)
                        .await
                        .map_err(|err| {
                            anyhow::anyhow!("failed to get pool data item at {height}: {err}")
                        })
                })
                .await
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
