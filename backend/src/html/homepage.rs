use super::{MicroUnit, RenderTimeTracer, TOOLTIPS};
use crate::to_badgateway;
use crate::utils::*;
use crate::State;
use askama::Template;
use futures_util::StreamExt;
use num_traits::{Inv, ToPrimitive};
use themelio_stf::{melvm::covenant_weight_from_bytes, PoolKey};
use themelio_structs::{Denom, Header, NetID};
use tide::Response;

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

#[derive(serde::Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
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

  