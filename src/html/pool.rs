use std::{str::FromStr, time::Duration};

use super::{friendly_denom, RenderTimeTracer};
use crate::{notfound, to_badgateway, to_badreq};
use anyhow::Context;
use askama::{filters::upper, Template};
use chrono::{NaiveDate, NaiveDateTime};
use futures_util::stream::FuturesUnordered;
use num_traits::ToPrimitive;
use serde::Serialize;
use smol::prelude::*;
use themelio_nodeprot::{ValClient, ValClientSnapshot};
use themelio_stf::{BlockHeight, Denom, MICRO_CONVERTER, NetID, PoolKey};
use async_trait::async_trait;

#[derive(Template)]
#[template(path = "pool.html")]
struct PoolTemplate {
    testnet: bool,
    friendly_denom: String,
    denom: String,
    last_item: PoolDataItem,
}

#[derive(Serialize, Clone)]
pub struct PoolDataItem {
    date: chrono::NaiveDateTime,
    height: u64,
    price: f64,
    liquidity: f64,
}

#[async_trait]
pub trait AsPoolDataItem {
    async fn as_pool_data_item(&self, denom: Denom) -> tide::Result<PoolDataItem>;
    async fn get_older_pool_data_item(&self, denom: Denom, height: u64) -> tide::Result<PoolDataItem>;
}


#[async_trait]
impl AsPoolDataItem for ValClientSnapshot {
    // returns a PoolDataItem that assumes the snapshot represents the most recent block
    async fn as_pool_data_item(&self, denom: Denom) -> tide::Result<PoolDataItem> {
        let height = self.current_header().height.0;
        let pool_key = PoolKey::mel_and(denom);
        let pool_info = self
            .get_pool(pool_key)
            .await?
            .ok_or_else(notfound)?;
        let price = pool_info.implied_price().to_f64().unwrap_or_default();
        let price = 
        if denom == pool_key.left {
            1.0 / price
        } else {
            price
        };
        let liquidity = 
        if denom == pool_key.left {
            pool_info.rights
        } else {
            pool_info.lefts
        } as f64
            * 2.0
            / MICRO_CONVERTER as f64;
        Ok(PoolDataItem {
            date: PoolDataItem::block_time(0),
            height: height,
            price,
            liquidity,
        })
    }
    async fn get_older_pool_data_item(&self, denom: Denom, height: u64) -> tide::Result<PoolDataItem>{
        let last_height = self.current_header().height.0;
        let snapshot = self.get_older(height.into())
        .await.map_err(to_badgateway)?;
        let mut item = snapshot.as_pool_data_item(denom).await?;
        Ok(item.set_time(last_height - height).clone())
    }

}

impl PoolDataItem {
    pub fn block_time(distance_from_now: u64) -> NaiveDateTime{
        chrono::Utc::now()
                .checked_sub_signed(
                    chrono::Duration::from_std(Duration::from_secs(30) * (distance_from_now) as u32)
                        .unwrap(),
                )
                .unwrap()
                .naive_utc()
    }
    pub fn set_time(&mut self, distance_from_now: u64) -> &mut Self {
        self.date = PoolDataItem::block_time(distance_from_now);
        self
    }
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
                snapshot.get_older_pool_data_item(denom, height).await
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


#[tracing::instrument(skip(req))]
pub async fn get_poolpage(req: tide::Request<ValClient>) -> tide::Result<tide::Body> {
    let _render = RenderTimeTracer::new("poolpage");
    let denom = req.param("denom").map(|v| v.to_string())?;
    let denom = Denom::from_str(&denom).map_err(to_badreq)?;

    let snapshot = req.state().snapshot().await.map_err(to_badgateway)?;
    let last_day = snapshot.as_pool_data_item(denom).await?;

    let mut body: tide::Body = PoolTemplate {
        testnet: req.state().netid() == NetID::Testnet,
        denom: denom.to_string(),
        friendly_denom: friendly_denom(denom),
        last_item: last_day.clone(),
    }
    .render()
    .unwrap()
    .into();
    body.set_mime("text/html");
    Ok(body)
}




