use std::ops::Range;

use anyhow::Context;
use futures_util::future::join_all;
use melstructs::{Block, BlockHeight, CoinData, CoinID, Transaction, TxHash};
use moka::sync::Cache;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::globals::{BACKEND, CLIENT};

/// A "crawl" of coin activity around a particular transaction. Coins are represented as string CoinIDs.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CoinCrawl {
    pub crawls: Vec<CrawlItem>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CrawlItem {
    coinid: CoinID,
    coindata: CoinData,
    coinheight: BlockHeight,
    spender: Option<(BlockHeight, TxHash)>,
}

impl CoinCrawl {
    /// Create a coin crawl surrounding the given TxHash and height.
    pub async fn crawl(height: BlockHeight, txhash: TxHash) -> anyhow::Result<Self> {
        static CACHE: Lazy<Cache<TxHash, CoinCrawl>> = Lazy::new(|| Cache::new(10000));
        if let Some(res) = CACHE.get(&txhash) {
            Ok(res)
        } else {
            let snap = CLIENT.snapshot(height).await?;
            let transaction = snap
                .get_transaction(txhash)
                .await?
                .context("transaction not found at this snap")?;

            // first, we know that the given transaction spent all of its inputs
            let input_crawls = join_all(transaction.inputs.clone().into_iter().map(|coinid| {
                let coindata_fut = snap.get_coin_spent_here(coinid);
                async move {
                    let coindata = coindata_fut.await?.context("must be spent here")?;
                    // also get the content
                    anyhow::Ok(CrawlItem {
                        coinid,
                        coindata: coindata.coin_data,
                        coinheight: coindata.height,
                        spender: Some((height, txhash)),
                    })
                }
            }))
            .await
            .into_iter();

            // but we want to know exactly who spent all the other things too.
            let chain_height = CLIENT.latest_snapshot().await?.current_header().height;
            let height_range = height.0..chain_height.0;
            let output_range = 0..transaction.outputs.len();
            let output_crawls = join_all(output_range.map(|i| {
                let coinid = transaction.output_coinid(i as u8).to_owned();
                let coindata = transaction.outputs[i].clone();
                let spender_fut = find_spend_within_range(coinid, height_range.clone());
                async move {
                    let spender = spender_fut.await?;
                    anyhow::Ok(CrawlItem {
                        coinid,
                        coindata,
                        coinheight: height,
                        spender, // None if unspent
                    })
                }
            }))
            .await
            .into_iter();

            let crawls = Self {
                crawls: input_crawls
                    .chain(output_crawls)
                    .collect::<Result<Vec<_>, _>>()?,
            };
            // ONLY cache if all the coins are spent. This prevents us from caching stale things
            if crawls.crawls.iter().all(|c| c.spender.is_some()) {
                CACHE.insert(txhash, crawls.clone());
            }
            Ok(crawls)
        }
    }
}

async fn find_spend_within_range(
    coinid: CoinID,
    height_range: Range<u64>,
) -> anyhow::Result<Option<(BlockHeight, TxHash)>> {
    static CACHE: Lazy<Cache<CoinID, (BlockHeight, TxHash)>> = Lazy::new(|| Cache::new(100000));

    if let Some(existant) = CACHE.get(&coinid) {
        Ok(Some(existant))
    } else {
        let range = height_range;
        let spend_height = match find_spending_height(coinid, range).await? {
            Some(spend) => spend,
            None => return Ok(None),
        };

        let snapshot = BACKEND.client.snapshot(spend_height).await?;
        let block = snapshot.current_block().await?;

        let spend_tx = find_spending_transaction(block, coinid).await?;
        let spend_txhash = spend_tx
            .context("Unexpected Failure: couldn't find spending transaction in spending block")?
            .hash_nosigs();
        CACHE.insert(coinid, (spend_height, spend_txhash));
        Ok(Some((spend_height, spend_txhash)))
    }
}

async fn find_spending_height(
    coin: CoinID,
    height_range: Range<u64>,
) -> anyhow::Result<Option<BlockHeight>> {
    let height_range: Vec<u64> = height_range.collect();

    if height_range.is_empty() {
        return Ok(None);
    }

    let index = {
        let mut max_index = height_range.last().unwrap().to_owned(); // will never be None since the range is not empty
        let mut min_index = height_range.first().unwrap().to_owned();
        while max_index - min_index > 1 {
            // println!("Bounding heights: {} {}", min_index, max_index);

            let check_index = (max_index - min_index) / 2 + min_index;
            let spent = is_spent(check_index, &coin).await?;

            // println!("Checking: {} Spent: {}", check_index, spent);

            match spent {
                true => max_index = check_index,
                false => min_index = check_index,
            }
        }
        max_index
    };

    let spend_edge: Vec<BlockHeight> = join_all([index - 1, index].map(|index| {
        async move {
            let spend_height = BlockHeight(index);
            let spend_coin_data = BACKEND.get_coin_at_height(spend_height, coin).await?; // would be nice to replace with a more lightweight function

            // println!("Spent here? {} {:?}", spend_height, spend_coin_data);
            anyhow::Ok(match spend_coin_data {
                Some(_) => None,
                None => Some(spend_height),
            })
        }
    }))
    .await
    .into_iter()
    .flatten()
    .flatten()
    .collect();

    match spend_edge.len() {
        1 => Ok(Some(spend_edge[0])),
        0 => Ok(None),
        // the binary search above is convergent and always terminates
        // `spend_edge_result` always has at most 2 values and values are never added
        _ => unreachable!(),
    }
}

async fn find_spending_transaction(
    block: Block,
    coinid: CoinID,
) -> anyhow::Result<Option<Transaction>> {
    let tx = block
        .transactions
        .iter()
        .find(|&tx| tx.inputs.clone().into_iter().any(|input| coinid == input))
        .cloned();
    Ok(tx)
}
// if the coin is found at the current height it is not spent; assumes coin existed
async fn is_spent(height: u64, coinid: &CoinID) -> anyhow::Result<bool> {
    let coin = BACKEND
        .get_coin_at_height(BlockHeight(height), *coinid)
        .await?;

    Ok(coin.is_none())
}
