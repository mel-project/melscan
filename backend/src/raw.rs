use std::sync::Arc;

use dashmap::DashMap;
use melblkidx::{BalanceTracker, Indexer};
use serde::{Deserialize, Serialize};
use smol::{lock::Semaphore, prelude::*};
use themelio_nodeprot::ValClient;
use themelio_stf::melvm::covenant_weight_from_bytes;
use themelio_structs::{
    Block, BlockHeight, CoinDataHeight, CoinID, CoinValue, Denom, Header, PoolKey, Transaction,
    TxHash,
};
use tmelcrypt::HashVal;

use crate::utils::*;
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
    pub total_fees: CoinValue,
    pub fee_multiplier: f64,
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
            total_fees: block.transactions.iter().map(|v| v.fee).sum(),
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

/// A Backend encapsulates the current state of a blockchain and exposes methods that are convenient to call from JSON-returning APIs.
#[derive(Clone)]
pub struct Backend {
    client: ValClient,

    indexer: Option<Arc<Indexer>>,
    supply_cache: Arc<DashMap<Denom, Arc<BalanceTracker>>>,
}

impl Backend {
    /// Creates a new Backend that wraps around a given ValClient.
    pub fn new(client: ValClient, indexer: Option<Indexer>) -> Self {
        Self {
            client,
            indexer: indexer.map(Arc::new),
            supply_cache: Default::default(),
        }
    }

    /// Obtains the latest indexed height.
    pub fn indexed_highest(&self) -> BlockHeight {
        self.indexer
            .as_ref()
            .map(|idx| idx.max_height().into())
            .unwrap_or_default()
    }

    /// Obtains the latest blockchain header.
    pub async fn get_latest_header(&self) -> anyhow::Result<Header> {
        Ok(self.client.snapshot().await?.current_header())
    }

    /// Get "overview" information at either the latest height or a given height.
    pub async fn get_overview(&self, height: Option<BlockHeight>) -> anyhow::Result<Overview> {
        let last_snap = match height {
            Some(height) => self.client.older_snapshot(height).await?,
            None => self.client.snapshot().await?,
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

    /// Obtains a specific transaction at a particular height.
    pub async fn get_transaction_at_height(
        &self,
        height: BlockHeight,
        txhash: TxHash,
    ) -> anyhow::Result<Option<Transaction>> {
        Ok(self
            .client
            .older_snapshot(height)
            .await?
            .get_transaction(txhash)
            .await?)
    }

    /// Obtains a particular coin at a particular height.
    pub async fn get_coin_at_height(
        &self,
        height: BlockHeight,
        coinid: CoinID,
    ) -> anyhow::Result<Option<CoinDataHeight>> {
        let older = self.client.older_snapshot(height).await?;
        Ok(older.get_coin(coinid).await?)
    }

    /// Gets a particular block.
    pub async fn get_block(&self, height: BlockHeight) -> anyhow::Result<Option<Block>> {
        let snap = self.client.snapshot().await?;
        if height > snap.current_header().height {
            return Ok(None);
        }
        Ok(Some(snap.get_older(height).await?.current_block().await?))
    }

    /// Gets a block summary.
    pub async fn get_block_summary(
        &self,
        height: BlockHeight,
    ) -> anyhow::Result<Option<BlockSummary>> {
        let snap = self.client.snapshot().await?;
        if height > snap.current_header().height {
            return Ok(None);
        }
        let snap = snap.get_older(height).await?;
        let proposer_reward = snap.get_proposer_reward().await?;
        Ok(Some(BlockSummary::from_block(
            snap.get_older(height).await?.current_block().await?,
            proposer_reward,
        )))
    }

    /// Gets the coin supply of the given denomination, at a given height. Only available if we have an indexer.
    pub async fn get_coin_supply(
        &self,
        height: BlockHeight,
        denom: Denom,
    ) -> anyhow::Result<Option<CoinValue>> {
        static SEMAPHORE: Semaphore = Semaphore::new(4);

        let _guard = SEMAPHORE.acquire().await;
        if let Some(indexer) = self.indexer.as_ref() {
            // get a balance tracker for this denom from the cache, or make a new one and put it into the cache
            let tracker = self
                .supply_cache
                .entry(denom)
                .or_insert_with(|| indexer.query_coins().denom(denom).balance_tracker().into())
                .value()
                .clone();
            let b = tracker.balance_at(height.0);
            eprintln!("got {} => {:?}", height, b);
            smol::future::yield_now().await;
            Ok(b)
        } else {
            Ok(None)
        }
    }
}

// /// Gets the latest blockchain header.
// pub async fn get_latest_header(client: ValClient) -> anyhow::Result<Header> {
//     let last_snap = client.snapshot().await?;
//     Ok(last_snap.current_header())
// }

// /// Gets a particular transaction at a height
// pub async fn get_transaction(
//     client: &ValClient,
//     height: u64,
//     txhash: String,
// ) -> anyhow::Result<Option<Transaction>> {
//     let txhash: TxHash = TxHash(txhash.parse()?);
//     let last_snap = client.snapshot().await?;
//     let older = last_snap.get_older(height.into()).await?;
//     Ok(older.get_transaction(txhash).await?)
// }

// /// Gets a particular coin at a height
// pub async fn get_coin(
//     client: ValClient,
//     height: u64,
//     coinid_string: String,
// ) -> anyhow::Result<Option<CoinDataHeight>> {
//     let coinid_exploded: Vec<&str> = coinid_string.split('-').collect();
//     if coinid_exploded.len() != 2 {
//         return Err(anyhow::format_err!("bad coinid"));
//     }
//     let txhash: Vec<u8> = hex::decode(&coinid_exploded[0])?;
//     let txhash: TxHash = HashVal(
//         txhash
//             .try_into()
//             .map_err(|_| anyhow::anyhow!("not the right length"))?,
//     )
//     .into();
//     let index: u8 = coinid_exploded[1].parse()?;
//     let older = client.older_snapshot(height).await?;
//     Ok(older.get_coin(CoinID { txhash, index }).await?)
// }

// /// Get a particular block
// pub async fn get_full_block(client: ValClient, height: u64) -> anyhow::Result<Block> {
//     let older = client.older_snapshot(height).await?;
//     let block = older.current_block().await?;
//     Ok(block)
// }

// /// Get block summary
// pub async fn get_block_summary(client: ValClient, height: u64) -> anyhow::Result<BlockSummary> {
//     let older = client.older_snapshot(height).await?;
//     let block = older.current_block().await?;
//     let proposer_reward = older.get_proposer_reward().await?;
//     Ok(BlockSummary::from_block(block, proposer_reward))
// }

// pub async fn get_pool(
//     client: ValClient,
//     height: u64,
//     left: Denom,
//     right: Denom,
// ) -> anyhow::Result<Option<PoolInfo>> {
//     let older = client.older_snapshot(height).await?;
//     let key = PoolKey { left, right };
//     let pool_state = older
//         .get_pool(key)
//         .await?
//         .context("Unable to get pool state")?;
//     let latest_item = older
//         .as_pool_data_item(key)
//         .await?
//         .context("Unable to get pool data item")?;
//     Ok(Some(PoolInfo {
//         pool_state,
//         latest_item,
//     }))
// }

// /// Obtains some pooldatas for the given range
// pub async fn get_pooldata_range(
//     client: ValClient,
//     left: Denom,
//     right: Denom,
//     lower_block: u64,
//     upper_block: u64,
// ) -> anyhow::Result<Vec<PoolDataItem>> {
//     static CACHE: Lazy<AsyncCache> = Lazy::new(|| AsyncCache::new(1_000_000));

//     let pool_key = PoolKey { left, right };
//     let snapshot = &client.snapshot().await?;

//     let blockheight_interval = {
//         if lower_block == upper_block {
//             vec![lower_block]
//         } else {
//             interpolate_between(lower_block, upper_block, 1000)
//                 .filter(|x| *x > 0)
//                 .collect()
//         }
//     };

//     let semaphore = Arc::new(Semaphore::new(128));
//     let mut item_futs = FuturesUnordered::new();
//     for height in &blockheight_interval {
//         let semaphore = semaphore.clone();
//         item_futs.push(async move {
//             let _guard = semaphore.acquire().await;
//             let cache_key = PoolInfoKey(pool_key, BlockHeight(*height));
//             CACHE
//                 .get_or_try_fill(&cache_key, async {
//                     snapshot
//                         .get_older_pool_data_item(pool_key, *height)
//                         .await
//                         .map_err(|err| {
//                             anyhow::anyhow!("failed to get pool data item at {height}: {err}")
//                         })
//                 })
//                 .await
//         });
//     }
//     // Gather the stuff
//     let mut output = vec![];
//     while let Some(res) = item_futs.next().await {
//         log::debug!(
//             "loading pooldata {}/{}",
//             output.len() + 1,
//             blockheight_interval.len()
//         );
//         if let Some(res) = res? {
//             output.push(res);
//         }
//     }
//     output.sort_unstable_by_key(|v| v.block_height());

//     Ok(output)
// }
