use std::{ str::FromStr, time::Duration};

use super::{friendly_denom, RenderTimeTracer};
use super::{InfoBubble, TOOLTIPS};
use crate::{notfound, to_badgateway, to_badreq, State};
use askama::{ Template};
use chrono::{NaiveDateTime};
use num_traits::ToPrimitive;
use serde::Serialize;
use themelio_nodeprot::{ValClientSnapshot};
use themelio_stf::{Denom, NetID, PoolKey, MICRO_CONVERTER};

use async_trait::async_trait;

#[derive(Template)]
#[template(path = "pool.html", escape = "none")]
struct PoolTemplate {
    testnet: bool,
    friendly_denom: String,
    pool_key: PoolKey,
    last_item: PoolDataItem,
    tooltips: &'static TOOLTIPS,
    denom_tooltip: &'static InfoBubble,
}

// 2 million cached pooldataitems is 64 mb
// 1 item is 256 bits
#[derive(Serialize, Clone)]
pub struct PoolDataItem {
    date: chrono::NaiveDateTime,
    height: u64,
    price: f64,
    liquidity: f64,
}

#[async_trait]
pub trait AsPoolDataItem {
    async fn as_pool_data_item(&self, pool_key: PoolKey) -> tide::Result<PoolDataItem>;
    async fn get_older_pool_data_item(
        &self,
        pool_key: PoolKey,
        height: u64,
    ) -> tide::Result<PoolDataItem>;
}

#[async_trait]
impl AsPoolDataItem for ValClientSnapshot {
    // returns a PoolDataItem that assumes the snapshot represents the most recent block
    async fn as_pool_data_item(&self, pool_key: PoolKey) -> tide::Result<PoolDataItem> {
        let height = self.current_header().height.0;
        let pool_info = self.get_pool(pool_key).await?.ok_or_else(notfound)?;
        let price = pool_info.implied_price().to_f64().unwrap_or_default();
        let liquidity = pool_info.lefts as f64 * 2.0 / MICRO_CONVERTER as f64;
        Ok(PoolDataItem {
            date: PoolDataItem::block_time(0),
            height: height,
            price,
            liquidity,
        })
    }
    async fn get_older_pool_data_item(
        &self,
        pool_key: PoolKey,
        height: u64,
    ) -> tide::Result<PoolDataItem> {
        let last_height = self.current_header().height.0;
        let snapshot = self.get_older(height.into()).await.map_err(to_badgateway)?;
        let mut item = snapshot.as_pool_data_item(pool_key).await?;
        Ok(item.set_time(last_height - height).clone())
    }
}

impl PoolDataItem {
    pub fn block_time(distance_from_now: u64) -> NaiveDateTime {
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
    pub fn block_height(&self) -> u64{
        self.height
    }
}



#[tracing::instrument(skip(req))]
pub async fn get_poolpage(req: tide::Request<State>) -> tide::Result<tide::Body> {
    let _render = RenderTimeTracer::new("poolpage");
    let pool_key = {
        let denom = req.param("denom_left").map(|v| v.to_string())?;
        let left = Denom::from_str(&denom).map_err(to_badreq)?;
        let denom = req.param("denom_right").map(|v| v.to_string())?;
        let right = Denom::from_str(&denom).map_err(to_badreq)?;
        PoolKey { left, right }
    };

    let friendly_denom = friendly_denom(pool_key.right);
    let snapshot = req
        .state()
        .val_client
        .snapshot()
        .await
        .map_err(to_badgateway)?;
    let last_day = snapshot.as_pool_data_item(pool_key).await?;

    let pool_template = PoolTemplate {
        testnet: req.state().val_client.netid() == NetID::Testnet,
        pool_key: pool_key,
        denom_tooltip: &TOOLTIPS[&friendly_denom],
        friendly_denom: friendly_denom,
        last_item: last_day,
        tooltips: &TOOLTIPS,
    };
    let mut body: tide::Body = pool_template.render().unwrap().into();
    body.set_mime("text/html");
    println!("{}", pool_template.tooltips[&pool_template.friendly_denom]);
    Ok(body)
}
