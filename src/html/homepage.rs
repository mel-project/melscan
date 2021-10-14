use crate::to_badgateway;
use askama::Template;
use futures_util::stream::FuturesOrdered;
use futures_util::StreamExt;
use num_traits::{Inv, ToPrimitive};
use themelio_nodeprot::ValClient;
use themelio_stf::{CoinID, Denom, Header, NetID, PoolKey};
use tide::Body;
use super::{MicroUnit, RenderTimeTracer};
use crate::utils::*;
#[derive(Template, serde::Serialize)]
#[template(path = "homepage.html")]
struct HomepageTemplate {
    testnet: bool,
    blocks: Vec<BlockSummary>,
    pool: PoolSummary,
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
pub async fn get_homepage(req: tide::Request<ValClient>) -> tide::Result<Body> {
    let _render = RenderTimeTracer::new("homepage");

    let last_snap = req.state().snapshot().await.map_err(to_badgateway)?;
    let mut blocks = Vec::new();
    let mut futs = get_old_blocks(&last_snap, 30);

    while let Some(inner) = futs.next().await {
        let (block, reward) = inner.map_err(to_badgateway)?;
        let mut transactions: Vec<TransactionSummary> = Vec::new();

        // push transactions
        for transaction in &block.transactions {
            if transactions.len() < 30 {
                transactions.push(TransactionSummary {
                    hash: hex::encode(&transaction.hash_nosigs().0),
                    shorthash: hex::encode(&transaction.hash_nosigs().0[0..5]),
                    height: block.header.height.0,
                    _weight: transaction.weight(),
                    mel_moved: MicroUnit(
                        transaction
                            .outputs
                            .iter()
                            .map(|v| if v.denom == Denom::Mel { v.value.0 } else { 0 })
                            .sum::<u128>()
                            + transaction.fee.0,
                        "MEL".into(),
                    ),
                })
            }
        }
        blocks.push(BlockSummary { 
            header: block.header,
            total_weight: block.transactions.iter().map(|v| v.weight()).sum(),
            reward_amount: MicroUnit(reward.into(), "MEL".into()),
            transactions: transactions.clone(),
        });
    }

    let mel_per_dosc = (last_snap
        .get_pool(PoolKey::new(Denom::Mel, Denom::NomDosc))
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

    let mut body: Body = HomepageTemplate {
        testnet: req.state().netid() == NetID::Testnet,
        blocks,
        pool,
    }
    .render()
    .unwrap()
    .into();
    body.set_mime("text/html");
    Ok(body)
}
