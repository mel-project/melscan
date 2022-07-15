use chrono::Utc;
use futures_util::Future;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use themelio_structs::{BlockHeight, Denom, PoolKey};

mod helpers;

/// A JSON-friendly graphing datum
#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct GraphDatum {
    pub height: BlockHeight,
    pub date: chrono::DateTime<Utc>,
    pub value: f64,
}

/// Given a range of blocks, and a function that resolves a value given a block height, return a bunch of graph data.
pub async fn graph_range<F: Future<Output = anyhow::Result<GraphDatum>> + Send + Sync + 'static>(
    start: BlockHeight,
    end: BlockHeight,
    approx_count: usize,
    resolve: impl Fn(BlockHeight) -> F,
    cached: impl Fn(BlockHeight) -> Option<GraphDatum>,
) -> anyhow::Result<Vec<GraphDatum>> {
    let heights = helpers::interpolate_between(start.0, end.0, approx_count as _).collect_vec();
    helpers::fast_async_map(heights, |height| {
        let height = BlockHeight(height);
        if let Some(cached) = cached(height) {
            Ok(Ok(cached))
        } else {
            Err(smolscale::spawn(resolve(height)))
        }
    })
    .await
    .into_iter()
    .collect()
}
