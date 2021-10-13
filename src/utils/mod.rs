use futures_util::{Future, stream::FuturesOrdered};
use themelio_nodeprot::ValClientSnapshot;
use themelio_stf::{Block, CoinID, CoinValue};
use crate::to_badgateway;


pub fn get_old_blocks(last_snap: &ValClientSnapshot, depth: u64) 
-> FuturesOrdered<impl Future<Output = anyhow::Result<(Block, CoinValue)>>> {
    let mut futs = FuturesOrdered::new();
    for height in (depth..=last_snap.current_header().height.0).rev().take(30) {
        let last_snap = last_snap.clone();
        futs.push(async move {
            log::debug!("rendering block {}", height);
            let old_snap = last_snap
                .get_older(height.into())
                .await?;
            let reward_coin = old_snap
                .get_coin(CoinID::proposer_reward(height.into()))
                .await?;
            let reward_amount = reward_coin.map(|v| v.coin_data.value).unwrap_or_default();
            let old_block = old_snap.current_block().await?;
            Ok((old_block, reward_amount))
        });
    }
    futs
}
