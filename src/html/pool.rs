use std::time::Duration;

use crate::{notfound, to_badgateway, to_badreq};
use anyhow::Context;
use askama::{Template, filters::upper};
use futures_util::stream::FuturesUnordered;
use num_traits::ToPrimitive;
use serde::Serialize;
use smol::prelude::*;
use themelio_nodeprot::{ValClient, ValClientSnapshot};
use themelio_stf::{Denom, NetID, PoolKey, MICRO_CONVERTER};
use super::{friendly_denom, RenderTimeTracer};


#[derive(Template)]
#[template(path = "pool.html")]
struct PoolTemplate {
    testnet: bool,
    denom: String,
    last_item: PoolDataItem,
}

#[tracing::instrument(skip(req))]
pub async fn get_poolpage(req: tide::Request<ValClient>) -> tide::Result<tide::Body> {
    let _render = RenderTimeTracer::new("poolpage");
    let denom = req.param("denom").map(|v| v.to_string())?;
    let denom = Denom::from_bytes(&hex::decode(&denom).map_err(to_badreq)?)
        .ok_or_else(|| to_badreq(anyhow::anyhow!("bad")))?;
    let last_day = pool_items(req.state(),1, 1, 1, denom).await?;
    let mut body: tide::Body = PoolTemplate {
        testnet: req.state().netid() == NetID::Testnet,
        denom: friendly_denom(denom),
        last_item: last_day.last().context("no last")?.clone(),
    }
    .render()
    .unwrap()
    .into();
    body.set_mime("text/html");
    Ok(body)
}

pub async fn pool_items(
    client: &ValClient,
    lower_block: u64,
    upper_block: u64,
    num_blocks: u64,
    denom: Denom,


) -> tide::Result<Vec<PoolDataItem>> {
    let snapshot = client.snapshot().await.map_err(to_badgateway)?;
    let last_height = snapshot.current_header().height.0;
    let blocks = upper_block - lower_block;
    let DIVIDER: u64 = num_blocks;
    // at most DIVIDER points
    let snapshot = &snapshot;
    let mut item_futs = FuturesUnordered::new();
    for height in (lower_block..= upper_block)
        .rev()
        .step_by((blocks / DIVIDER) as usize)
    {
        item_futs.push(async move {
            let old_snap = snapshot
                .get_older(height.into())
                .await
                .map_err(to_badgateway)?;
            let pool_key = PoolKey::mel_and(denom);
            let pool_info = old_snap
                .get_pool(pool_key)
                .await
                .map_err(to_badgateway)?
                .ok_or_else(notfound)?;
            let price = pool_info.implied_price().to_f64().unwrap_or_default();
            let price = if denom == pool_key.left {
                1.0 / price
            } else {
                price
            };
            let liquidity = if denom == pool_key.left {
                pool_info.rights
            } else {
                pool_info.lefts
            } as f64
                * 2.0
                / MICRO_CONVERTER as f64;
            Ok::<_, tide::Error>(PoolDataItem {
                date: chrono::Utc::now()
                    .checked_sub_signed(
                        chrono::Duration::from_std(
                            Duration::from_secs(30) * (last_height - height) as u32,
                        )
                        .unwrap(),
                    )
                    .unwrap()
                    .naive_utc(),
                height,
                price,
                liquidity,
            })
        })
    }
    // Gather the stuff
    let mut output = vec![];
    while let Some(res) = item_futs.next().await {
        log::debug!("loading pooldata {}/{}", output.len(), DIVIDER);
        output.push(res?);
    }
    output.sort_unstable_by_key(|v| v.height);
    Ok(output)
}

#[derive(Serialize, Clone)]
pub struct PoolDataItem {
    date: chrono::NaiveDateTime,
    height: u64,
    price: f64,
    liquidity: f64,
}
