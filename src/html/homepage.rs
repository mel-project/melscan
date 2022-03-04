use super::{MicroUnit, RenderTimeTracer, TOOLTIPS};
use crate::to_badgateway;
use crate::utils::*;
use crate::State;
use askama::Template;
use futures_util::StreamExt;
use num_traits::{Inv, ToPrimitive};
use themelio_stf::{melvm::covenant_weight_from_bytes, PoolKey};
use themelio_structs::{Denom, Header, NetID};
use tide::Body;

#[derive(Template)]
#[template(path = "homepage.html", escape = "none")]
struct HomepageTemplate {
    testnet: bool,
    blocks: Vec<BlockSummary>,
    pool: PoolSummary,
    tooltips: &'static TOOLTIPS,
    transactions: Vec<TransactionSummary>,
}

#[derive(serde::Serialize)]
// A block summary for the homepage.
pub struct BlockSummary {
    pub header: Header,
    pub total_weight: u128,
    pub reward_amount: MicroUnit,
    pub transactions: Vec<TransactionSummary>,
}

#[derive(serde::Serialize, Clone)]
// A transaction summary for the homepage.
pub struct TransactionSummary {
    pub hash: String,
    pub shorthash: String,
    pub height: u64,
    pub _weight: u128,
    pub mel_moved: MicroUnit,
}

#[derive(serde::Serialize)]
// A pool summary for the homepage.
struct PoolSummary {
    mel_per_sym: f64,
    mel_per_dosc: f64,
}

/// Homepage
#[tracing::instrument(skip(req))]
pub async fn get_homepage(req: tide::Request<State>) -> tide::Result<Body> {
    let _render = RenderTimeTracer::new("homepage");

    let last_snap = req
        .state()
        .val_client
        .snapshot()
        .await
        .map_err(to_badgateway)?;
    let mut blocks = Vec::new();
    let mut futs = get_old_blocks(&last_snap, 50);

    while let Some(inner) = futs.next().await {
        let (block, reward) = inner.map_err(to_badgateway)?;
        let transactions: Vec<TransactionSummary> = get_transactions(&block);

        blocks.push(BlockSummary {
            header: block.header,
            total_weight: block
                .transactions
                .iter()
                .map(|v| v.weight(covenant_weight_from_bytes))
                .sum(),
            reward_amount: MicroUnit(reward.into(), "MEL".into()),
            transactions: transactions.clone(),
        });
    }

    let mel_per_dosc = (last_snap
        .get_pool(PoolKey::new(Denom::Mel, Denom::Erg))
        .await
        .map_err(to_badgateway)?
        .unwrap()
        .implied_price()
        .inv()
        * themelio_stf::dosc_inflator(last_snap.current_header().height))
    .to_f64()
    .unwrap_or_default();

    let pool = PoolSummary {
        mel_per_sym: last_snap
            .get_pool(PoolKey::new(Denom::Mel, Denom::Sym))
            .await
            .map_err(to_badgateway)?
            .unwrap()
            .implied_price()
            .to_f64()
            .unwrap_or_default(),
        mel_per_dosc,
    };

    let transactions = blocks
        .iter()
        .map(|b| b.transactions.iter())
        .flatten()
        .cloned()
        .take(50)
        .collect();

    let mut body: Body = HomepageTemplate {
        testnet: req.state().val_client.netid() == NetID::Testnet,
        blocks,
        pool,
        tooltips: &TOOLTIPS,
        transactions,
    }
    .render()
    .unwrap()
    .into();
    body.set_mime("text/html");
    Ok(body)
}
