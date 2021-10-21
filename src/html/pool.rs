use std::time::Duration;

use super::{friendly_denom, RenderTimeTracer};
use crate::{notfound, to_badgateway, to_badreq};
use anyhow::Context;
use askama::{filters::upper, Template};
use futures_util::stream::FuturesUnordered;
use num_traits::ToPrimitive;
use serde::Serialize;
use smol::prelude::*;
use themelio_nodeprot::{ValClient, ValClientSnapshot};
use themelio_stf::{Denom, NetID, PoolKey, MICRO_CONVERTER};

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

    let snapshot = req.state().snapshot().await.map_err(to_badgateway)?;
    let last_day = pool_item(&snapshot, denom).await?;
    let mut body: tide::Body = PoolTemplate {
        testnet: req.state().netid() == NetID::Testnet,
        denom: friendly_denom(denom),
        last_item: last_day.clone(),
    }
    .render()
    .unwrap()
    .into();
    body.set_mime("text/html");
    Ok(body)
}

pub async fn pool_item(snapshot: &ValClientSnapshot, denom: Denom) -> tide::Result<PoolDataItem> {
    let last_height = snapshot.current_header().height.0;
    let pool_key = PoolKey::mel_and(denom);
    let pool_info = snapshot
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
                chrono::Duration::from_std(Duration::from_secs(30) * (last_height) as u32)
                    .unwrap(),
            )
            .unwrap()
            .naive_utc(),
        height: last_height,
        price,
        liquidity,
    })

    // let items = pool_items(&client, last_height, last_height, 1, denom).await?;
    // Ok(Some(items.last().unwrap().clone()))
}
pub async fn pool_items(
    client: &ValClient,
    lower_block: u64,
    upper_block: u64,
    num_blocks: u64,
    denom: Denom,
) -> tide::Result<Vec<PoolDataItem>> {
    let snapshot = client.snapshot().await.map_err(to_badgateway)?;
    let blocks = upper_block - lower_block;
    let divider: u64 = num_blocks.min(upper_block - lower_block);
    // at most DIVIDER points
    let snapshot = &snapshot;
    let mut item_futs = FuturesUnordered::new();
    for height in (lower_block..=upper_block)
        .rev()
        .step_by((blocks / divider) as usize)
    {
        item_futs.push(
            async move {
                let snapshot = snapshot.get_older(height.into()).await.map_err(to_badgateway)?;
                pool_item(&snapshot, denom).await
            }
        );
    }
    // Gather the stuff
    let mut output = vec![];
    while let Some(res) = item_futs.next().await {
        log::debug!("loading pooldata {}/{}", output.len(), divider);
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
