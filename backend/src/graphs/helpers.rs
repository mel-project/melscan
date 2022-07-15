use futures_util::stream::FuturesUnordered;
use futures_util::StreamExt;
use itertools::Itertools;
use smol::Task;
use std::collections::BTreeMap;

/// Interpolates between two numbers in a cache-friendly fashion
pub fn interpolate_between(start: u64, end: u64, approx_count: u64) -> impl Iterator<Item = u64> {
    let interval = ((end - start).max(1) / approx_count).next_power_of_two();
    (start..=end).filter(move |i| i % interval == 0).unique()
}

/// Efficiently map an asynchronous function over a vector, doing any concurrency only when any of the functions need to block (indicated by returning a task instead of an immediate value).
pub async fn fast_async_map<T, U>(v: Vec<T>, f: impl Fn(T) -> Result<U, Task<U>>) -> Vec<U> {
    let mut toret = BTreeMap::new();
    let mut pending = FuturesUnordered::new();
    for (i, val) in v.into_iter().enumerate() {
        match f(val) {
            Ok(res) => {
                toret.insert(i, res);
            }
            Err(fut) => {
                pending.push(async move { (i, fut.await) });
            }
        }
    }
    while let Some((i, v)) = pending.next().await {
        toret.insert(i, v);
    }
    toret.into_iter().map(|d| d.1).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fam_simple() {
        assert_eq!(
            smol::future::block_on(fast_async_map(vec![1, 2, 3, 4, 5], |x| Ok(x * 2))),
            vec![2, 4, 6, 8, 10]
        )
    }

    #[test]
    fn fam_blocking() {
        assert_eq!(
            smol::future::block_on(fast_async_map(vec![1, 2, 3, 4, 5], |x| Err(smol::spawn(
                async move { x * 2 }
            )))),
            vec![2, 4, 6, 8, 10]
        )
    }
}
