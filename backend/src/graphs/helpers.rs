use futures_util::StreamExt;
use futures_util::{stream::FuturesUnordered, Future};
use itertools::Itertools;
use std::collections::BTreeMap;

/// Interpolates between two numbers in a cache-friendly fashion
pub fn interpolate_between(start: u64, end: u64, approx_count: u64) -> impl Iterator<Item = u64> {
    let interval = ((end - start).max(1) / approx_count).next_power_of_two();
    (start..=end)
        .filter(move |i| i % interval == 0)
        .chain(std::iter::once(end))
        .unique()
}

/// Efficiently map an asynchronous function over a vector, doing any concurrency only when any of the asynchronous functions "block".
pub async fn fast_async_map<T, U, F: Future<Output = U> + Unpin>(
    v: Vec<T>,
    f: impl Fn(T) -> F,
) -> Vec<U> {
    let mut toret = BTreeMap::new();
    let mut pending = FuturesUnordered::new();
    for (i, val) in v.into_iter().enumerate() {
        let fut = f(val);
        if let Some(res) = smol::future::poll_once(&mut fut).await {
            toret.insert(i, val);
        } else {
            pending.push(async { (i, fut.await) });
        }
    }
    while let Some((i, v)) = pending.next().await {
        toret.insert(i, v);
    }
    toret.values().collect()
}
