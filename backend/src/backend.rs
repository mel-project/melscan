use std::{collections::BTreeMap, sync::Arc, time::Duration};

use anyhow::Context;
use chrono::Utc;
use dashmap::DashMap;
use itertools::Itertools;
use melblkidx::{BalanceTracker, Indexer};
use moka::sync::Cache;
use serde::{Deserialize, Serialize};
use smol::{lock::Semaphore, prelude::*};
use tap::Tap;
use themelio_nodeprot::ValClient;
use themelio_stf::melvm::covenant_weight_from_bytes;
use themelio_structs::{
    Address, Block, BlockHeight, CoinDataHeight, CoinID, CoinValue, Denom, Header, PoolKey,
    Transaction, TxHash,
};
use tmelcrypt::HashVal;

use crate::{graphs::height_to_datetime, utils::*};
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

/// A summary of a particular address.
#[derive(serde::Serialize, Clone, PartialEq, PartialOrd, Debug)]
pub struct AddressSummary {
    pub balances: BTreeMap<String, f64>,
    pub transactions: Vec<AddressTransactionSummary>,
}

#[derive(serde::Serialize, Clone, PartialEq, PartialOrd, Debug)]
pub struct AddressTransactionSummary {
    pub height: BlockHeight,
    pub date: chrono::DateTime<Utc>,
    pub txhash: TxHash,
    pub deltas: BTreeMap<String, f64>,
}

/// A Backend encapsulates the current state of a blockchain and exposes methods that are convenient to call from JSON-returning APIs.
#[derive(Clone)]
pub struct Backend {
    client: ValClient,

    indexer: Option<Arc<Indexer>>,
    supply_cache: Arc<DashMap<Denom, Arc<BalanceTracker>>>,

    address_summary_cache: Arc<Cache<Address, AddressSummary>>,
}

impl Backend {
    /// Creates a new Backend that wraps around a given ValClient.
    pub fn new(client: ValClient, indexer: Option<Indexer>) -> Self {
        Self {
            client,
            indexer: indexer.map(Arc::new),
            supply_cache: Default::default(),

            address_summary_cache: Arc::new(
                Cache::builder()
                    .max_capacity(10000)
                    .time_to_live(Duration::from_secs(30))
                    .build(),
            ),
        }
    }

    /// Obtains the latest indexed height.
    pub fn indexed_highest(&self) -> BlockHeight {
        self.indexer
            .as_ref()
            .map(|idx| idx.max_height())
            .unwrap_or_default()
    }

    /// Obtains the latest blockchain header.
    pub async fn get_latest_header(&self) -> anyhow::Result<Header> {
        Ok(self.client.snapshot().await?.current_header())
    }

    /// Searches for the transaction matching a given hash.
    pub async fn search_transaction(&self, txhash: TxHash) -> anyhow::Result<Option<BlockHeight>> {
        Ok(self
            .indexer
            .as_ref()
            .and_then(|i| i.txhash_to_height(txhash)))
    }

    /// Searches for the block matching a given hash.
    pub async fn search_block(&self, blkhash: HashVal) -> anyhow::Result<Option<BlockHeight>> {
        Ok(self
            .indexer
            .as_ref()
            .and_then(|i| i.blkhash_to_height(blkhash)))
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
        let this = self.clone();
        smol::unblock(move || {
            if let Some(indexer) = this.indexer.as_ref() {
                // get a balance tracker for this denom from the cache, or make a new one and put it into the cache
                let tracker = this
                    .supply_cache
                    .entry(denom)
                    .or_insert_with(|| indexer.query_coins().denom(denom).balance_tracker().into())
                    .value()
                    .clone();
                let b = tracker.balance_at(height.0);
                eprintln!("got {} => {:?}", height, b);
                Ok(b)
            } else {
                Ok(None)
            }
        })
        .await
    }

    /// Gets the total summary of some address. Only available if we have an indexer.
    pub async fn get_address_summary(&self, address: Address) -> anyhow::Result<AddressSummary> {
        let this = self.clone();
        smol::unblock(move || {
            this.address_summary_cache
                .try_get_with(address, || {
                    let indexer = this.indexer.as_ref().context("no indexer")?.clone();
                    let current_coins = indexer.query_coins().covhash(address).unspent();
                    let mut balances: BTreeMap<String, f64> = BTreeMap::new();
                    for coin in current_coins.iter() {
                        *balances
                            .entry(coin.coin_data.denom.to_string())
                            .or_default() += coin.coin_data.value.0 as f64 / 1_000_000.0;
                    }
                    // TODO more efficient way of doing this
                    let mut transactions: BTreeMap<(TxHash, BlockHeight), BTreeMap<String, f64>> =
                        BTreeMap::new();
                    for coin in indexer.query_coins().covhash(address).iter() {
                        let mapping = transactions
                            .entry((coin.create_txhash, coin.create_height))
                            .or_default();
                        // we credit the transaction that produced the coin
                        *mapping.entry(coin.coin_data.denom.to_string()).or_default() +=
                            coin.coin_data.value.0 as f64 / 1_000_000.0;
                        // and debit the transaction that spent the coin
                        if let Some(s) = coin.spend_info {
                            let mapping = transactions
                                .entry((s.spend_txhash, s.spend_height))
                                .or_default();
                            // we credit the transaction that produced the coin
                            *mapping.entry(coin.coin_data.denom.to_string()).or_default() -=
                                coin.coin_data.value.0 as f64 / 1_000_000.0;
                        }
                    }
                    anyhow::Ok(AddressSummary {
                        balances,
                        transactions: transactions
                            .into_iter()
                            .map(|(k, v)| AddressTransactionSummary {
                                height: k.1,
                                date: height_to_datetime(k.1),
                                deltas: v,
                                txhash: k.0,
                            })
                            .collect_vec()
                            .tap_mut(|v| v.sort_unstable_by_key(|v| v.height)),
                    })
                })
                .map_err(|e| anyhow::anyhow!("{:?}", e))
        })
        .await
    }

    /// Gets the leaderboard for a particular denomination.
    pub async fn get_leaderboard(&self, denom: Denom) -> anyhow::Result<BTreeMap<String, f64>> {
        let indexer = self.indexer.as_ref().context("no indexer")?;
        Ok(indexer.query_coins().unspent().denom(denom).iter().fold(
            BTreeMap::new(),
            |mut map, cinfo| {
                *map.entry(cinfo.coin_data.covhash.to_string()).or_default() +=
                    cinfo.coin_data.value.0 as f64 / 1_000_000.0;
                map
            },
        ))
    }
}
