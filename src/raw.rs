use std::str::FromStr;
use std::{convert::TryInto, sync::Arc};

use anyhow::Context;
use futures_util::stream::FuturesUnordered;
use themelio_stf::{melvm::covenant_weight_from_bytes, PoolKey};

use smol::{lock::Semaphore, prelude::*};
use themelio_structs::{BlockHeight, CoinID, Denom, TxHash};
use tide::Body;
use tmelcrypt::HashVal;

use crate::html::AsPoolDataItem;
use crate::html::{homepage::BlockSummary, MicroUnit};
use crate::utils::*;
use crate::{notfound, to_badgateway, to_badreq, State};

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct PoolInfoKey(PoolKey, BlockHeight);
/// Get the latest status
#[tracing::instrument(skip(req))]
pub async fn get_latest(req: tide::Request<State>) -> tide::Result<Body> {
    let last_snap = req
        .state()
        .val_client
        .snapshot()
        .await
        .map_err(to_badgateway)?;
    Body::from_json(&last_snap.current_header())
}

/// Get a particular block header
#[tracing::instrument(skip(req))]
pub async fn get_header(req: tide::Request<State>) -> tide::Result<Body> {
    let height: u64 = req.param("height")?.parse().map_err(to_badreq)?;
    let last_snap = req
        .state()
        .val_client
        .snapshot()
        .await
        .map_err(to_badgateway)?;
    let older = last_snap
        .get_history(height.into())
        .await
        .map_err(to_badgateway)?;
    Body::from_json(&older.ok_or_else(notfound)?)
}

/// Get a particular transaction
#[tracing::instrument(skip(req))]
pub async fn get_transaction(req: tide::Request<State>) -> tide::Result<Body> {
    let height: u64 = req.param("height")?.parse()?;
    let txhash: String = req.param("txhash")?.into();
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
        reward_amount: MicroUnit(reward_amount.into(), "MEL".into()),
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

    let semaphore = Arc::new(Semaphore::new(64));
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
        output.push(res?);
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
