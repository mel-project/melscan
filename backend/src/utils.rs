use crate::backend::TransactionSummary;
use anyhow::Context;
use futures_util::{stream::FuturesOrdered, Future};
use num_traits::ToPrimitive;
use themelio_nodeprot::ValClientSnapshot;
use themelio_stf::melvm::covenant_weight_from_bytes;
use themelio_structs::{Block, CoinID, CoinValue, Denom, PoolKey};

pub fn get_old_blocks(
    last_snap: &ValClientSnapshot,
    depth: usize,
) -> FuturesOrdered<impl Future<Output = anyhow::Result<(Block, CoinValue)>>> {
    let mut futs = FuturesOrdered::new();
    for height in (0..=last_snap.current_header().height.0).rev().take(depth) {
        let last_snap = last_snap.clone();
        futs.push(async move {
            // log::debug!("rendering block {}", height);
            let old_snap = last_snap.get_older(height.into()).await?;
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

pub fn get_transactions(block: &Block) -> Vec<TransactionSummary> {
    let mut transactions: Vec<TransactionSummary> = Vec::new();
    for transaction in &block.transactions {
        transactions.push(TransactionSummary {
            hash: hex::encode(&transaction.hash_nosigs().0),
            shorthash: hex::encode(&transaction.hash_nosigs().0[0..5]),
            height: block.header.height.0,
            weight: transaction.weight(covenant_weight_from_bytes),
            mel_moved: transaction
                .outputs
                .iter()
                .map(|v| if v.denom == Denom::Mel { v.value.0 } else { 0 })
                .sum::<u128>()
                + transaction.fee.0,
        })
    }
    transactions.sort_unstable();
    transactions
}

pub async fn get_exchange(
    last_snap: &ValClientSnapshot,
    denom1: Denom,
    denom2: Denom,
) -> anyhow::Result<f64> {
    let pool = last_snap
        .get_pool(PoolKey::new(denom1, denom2))
        .await
        .context(format!("Unable to get exchange for {denom1}/{denom2}"))?
        .unwrap();
    let micro = pool.implied_price().to_f64().unwrap_or_default();
    if denom1.to_bytes() < denom2.to_bytes() {
        Ok(1.0 / micro)
    } else {
        Ok(micro)
    }
}
