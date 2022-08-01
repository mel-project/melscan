use std::{collections::BTreeMap, ops::Range};

use anyhow::Context;
use futures_util::future::join_all;
use lazy_static::__Deref;
use serde::{Deserialize, Serialize};
use themelio_structs::{Block, BlockHeight, CoinData, CoinID, Transaction, TxHash};

use crate::globals::{BACKEND, CLIENT};

/// A "crawl" of coin activity around a particular transaction. Coins are represented as string CoinIDs.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CoinCrawl {
    pub coin_contents: Vec<(CoinID, CoinData)>,
    pub coin_spenders: BTreeMap<String, TxHash>,
}

impl CoinCrawl {
    /// Create a coin crawl surrounding the given TxHash and height.
    pub async fn crawl(height: BlockHeight, txhash: TxHash) -> anyhow::Result<Self> {
        
        let snap = CLIENT.older_snapshot(height).await?;
        let transaction = snap
            .get_transaction(txhash)
            .await?
            .context("transaction not found at this snap")?;
        
        
            let mut coin_contents: Vec<(CoinID, CoinData)> = vec![];
        let mut coin_spenders = BTreeMap::new();
        
        
        // first, we know that the given transaction spent all of its inputs
    for input in transaction.inputs.iter() {
            coin_spenders.insert(input.to_string(), transaction.hash_nosigs());

            let coindata = snap.get_coin_spent_here(*input)
            .await?
            .context("must be spent here")?
            .coin_data;
            // also get the content
            coin_contents.push(
                ( *input,             // What happens in a dereference ?
                 coindata )
            );
        }
        
        // but we want to know exactly who spent all the other things too.
        let chain_height = CLIENT.snapshot().await?.current_header().height;
        let height_range = height.0..chain_height.0;
        for (output_coinid, output_coindata) in (0..transaction.outputs.len())
            .map(|i| transaction.output_coinid(i as u8))
            .zip(transaction.outputs.iter())
        {
            coin_contents.push((output_coinid, output_coindata.clone()));
            let spend = find_spend_within_range(output_coinid, height_range.clone()).await?;
            if let Some(spend) = spend {
                coin_spenders.insert(output_coinid.to_string(), spend.txhash);
            }
        }


        Ok(Self {
            coin_contents,
            coin_spenders,
        })
    }
}

async fn find_spend_within_range(
    coinid: CoinID,
    height_range: Range<u64>,
) -> anyhow::Result<Option<CoinLocation>> {
    let range = height_range;
    let spend_height = match find_spending_height(coinid, range).await? {
        Some(spend) => spend,
        None => return Ok(None),
    };

    let snapshot = BACKEND.client.older_snapshot(spend_height).await?;
    let block = snapshot.current_block().await?;

    let spend_tx = find_spending_transaction(block, coinid).await?;
    let spend_txhash = spend_tx
        .context("Unexpected Failure: couldn't find spending transaction in spending block")?
        .hash_nosigs();
    println!("{coinid}{spend_height}");

    Ok(Some(CoinLocation {
        coinid,
        txhash: spend_txhash,
        height: spend_height,
        data: snapshot
            .get_coin_spent_here(coinid)
            .await?
            .context("this should not happen")?
            .coin_data,
    }))
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
#[derive(Clone, Debug, Serialize)]
struct CoinLocation {
    coinid: CoinID,
    txhash: TxHash,
    height: BlockHeight,
    data: CoinData,
}

// if the coin is found at the current height it is not spent; assumes coin existed
async fn is_spent(height: u64, coinid: &CoinID) -> anyhow::Result<bool> {
    let coin = BACKEND
        .get_coin_at_height(BlockHeight(height), *coinid)
        .await?;

    Ok(coin.is_none())
}
