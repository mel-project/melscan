use std::sync::Arc;

use chrono::{TimeZone, Utc};
use futures_util::Future;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use themelio_structs::BlockHeight;

mod helpers;

/// Convert datetime to a block height
pub fn datetime_to_height(dt: chrono::DateTime<Utc>) -> BlockHeight {
    BlockHeight((dt.timestamp() as u64).saturating_sub(1618365600) / 30)
}

/// Convert block height to a datetime
pub fn height_to_datetime(height: BlockHeight) -> chrono::DateTime<Utc> {
    Utc.timestamp((height.0 * 30 + 1618365600) as i64, 0)
}

/// A JSON-friendly graphing datum
#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct GraphDatum {
    pub height: BlockHeight,
    pub date: chrono::DateTime<Utc>,
    pub value: f64,
}

/// Given a range of blocks, and a function that resolves a value given a block height, return a bunch of graph data.
pub async fn graph_range<F: Future<Output = anyhow::Result<f64>> + Send + 'static>(
    start: BlockHeight,
    end: BlockHeight,
    approx_count: usize,
    resolve: impl Fn(BlockHeight) -> F + Send + Sync + 'static,
    cache_get: impl Fn(BlockHeight) -> Option<f64> + 'static,
    cache_set: impl Fn(BlockHeight, f64) + Send + Sync + 'static,
) -> anyhow::Result<Vec<GraphDatum>> {
    let heights = helpers::interpolate_between(start.0, end.0, approx_count as _).collect_vec();
    let resolve = Arc::new(resolve);
    let cache_set = Arc::new(cache_set);
    helpers::fast_async_map(heights, |height| {
        let height = BlockHeight(height);
        if let Some(cached) = cache_get(height) {
            Ok(Ok(GraphDatum {
                value: cached,
                height,
                date: height_to_datetime(height),
            }))
        } else {
            let resolve = resolve.clone();
            let cache_set = cache_set.clone();
            Err(smolscale::spawn(async move {
                let res = resolve(height).await?;
                cache_set(height, res);
                Ok(GraphDatum {
                    value: res,
                    height,
                    date: height_to_datetime(height),
                })
            }))
        }
    })
    .await
    .into_iter()
    .collect()
}
