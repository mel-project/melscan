use std::convert::TryInto;

use anyhow::Context;
use themelio_nodeprot::ValClient;
use themelio_stf::{CoinID, Denom, PoolKey, TxHash};

use tide::Body;
use tmelcrypt::HashVal;

use crate::html::{TransactionSummary, pool_items};
use crate::html::{homepage::BlockSummary, MicroUnit};
use crate::utils::*;
use crate::{notfound, to_badgateway, to_badreq};

/// Get the latest status
#[tracing::instrument(skip(req))]
pub async fn get_latest(req: tide::Request<ValClient>) -> tide::Result<Body> {
    let last_snap = req.state().snapshot().await.map_err(to_badgateway)?;
    Body::from_json(&last_snap.current_header())
}

/// Get a particular block header
#[tracing::instrument(skip(req))]
pub async fn get_header(req: tide::Request<ValClient>) -> tide::Result<Body> {
    let height: u64 = req.param("height")?.parse().map_err(to_badreq)?;
    let last_snap = req.state().snapshot().await.map_err(to_badgateway)?;
    let older = last_snap
        .get_history(height.into())
        .await
        .map_err(to_badgateway)?;
    Body::from_json(&older.ok_or_else(notfound)?)
}

/// Get a particular transaction
#[tracing::instrument(skip(req))]
pub async fn get_transaction(req: tide::Request<ValClient>) -> tide::Result<Body> {
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
    let last_snap = req.state().snapshot().await?;
    let older = last_snap.get_older(height.into()).await?;
    let tx = older.get_transaction(txhash).await?;
    Body::from_json(&tx.ok_or_else(notfound)?)
}

/// Get a particular coin
#[tracing::instrument(skip(req))]
pub async fn get_coin(req: tide::Request<ValClient>) -> tide::Result<Body> {
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
    let last_snap = req.state().snapshot().await?;
    let older = last_snap.get_older(height.into()).await?;
    let cdh = older.get_coin(dbg!(CoinID { txhash, index })).await?;
    Body::from_json(&cdh.ok_or_else(notfound)?)
}

/// Get a particular pool
#[tracing::instrument(skip(req))]
pub async fn get_pool(req: tide::Request<ValClient>) -> tide::Result<Body> {
    let height: u64 = req.param("height")?.parse()?;
    let denom_string: String = req.param("denom")?.into();
    let denom =
        Denom::from_bytes(&hex::decode(&denom_string).map_err(to_badreq)?).context("oh no")?;

    let last_snap = req.state().snapshot().await?;
    let older = last_snap.get_older(height.into()).await?;
    let cdh = older
        .get_pool(PoolKey::mel_and(denom))
        .await
        .map_err(to_badgateway)?;
    Body::from_json(&cdh.ok_or_else(notfound)?)
}

/// Get a particular block
#[tracing::instrument(skip(req))]
pub async fn get_full_block(req: tide::Request<ValClient>) -> tide::Result<Body> {
    let height: u64 = req.param("height")?.parse().map_err(to_badreq)?;
    let last_snap = req.state().snapshot().await.map_err(to_badgateway)?;
    let older = last_snap
        .get_older(height.into())
        .await
        .map_err(to_badgateway)?;
    let block = older.current_block().await.map_err(to_badgateway)?;
    Body::from_json(&block)
}

/// Get block summary
#[tracing::instrument(skip(req))]
pub async fn get_block_summary(req: tide::Request<ValClient>) -> tide::Result<Body> {
    let height: u64 = req.param("height")?.parse().map_err(to_badreq)?;
    let last_snap = req.state().snapshot().await.map_err(to_badgateway)?;
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

    let transactions = get_transactions(&block, 30);
    
    Body::from_json(&BlockSummary {
        header: block.header,
        total_weight: block.transactions.iter().map(|v| v.weight()).sum(),
        reward_amount: MicroUnit(reward_amount.into(), "MEL".into()),
        transactions
    })
}

pub async fn get_pooldata_range(req: tide::Request<ValClient>) -> tide::Result<Body> {
    let client = req.state();
    let snapshot = client.snapshot().await.map_err(to_badgateway)?;
    let height = 1000;
    let old_snap = snapshot
    .get_older(height.into())
    .await
    .map_err(to_badgateway)?;
    let denom = Denom::from_bytes(&hex::decode("73").map_err(to_badreq)?)
        .ok_or_else(|| to_badreq(anyhow::anyhow!("bad")))?;

    let pool = pool_items(&old_snap, denom,100 * 300, 300);

    Body::from_json(&pool.await?)
}