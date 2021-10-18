use crate::{to_badgateway, to_badreq};
use askama::Template;
use serde::__private::de::IdentifierDeserializer;
use themelio_nodeprot::ValClient;
use themelio_stf::{BlockHeight, CoinID, Header, NetID, TxHash};

use super::{MicroUnit, RenderTimeTracer, InfoBubble};

#[derive(Template)]
#[template(path = "block.html", escape = "none")]
struct BlockTemplate {
    testnet: bool,
    header: Header,
    txcount: usize,
    txweight: u128,
    txhashes: Vec<(TxHash, u128)>,

    fee_pool: MicroUnit,
    fee_multiplier: f64,
    _reward_amount: MicroUnit,
    total_fees: MicroUnit,
    tooltips: ToolTips
}

#[derive(serde::Deserialize)]
pub struct ToolTips {
    hash: InfoBubble,
    height: InfoBubble,
    fee_pool: InfoBubble,
}

#[tracing::instrument(skip(req))]
pub async fn get_blockpage(req: tide::Request<ValClient>) -> tide::Result<tide::Body> {
    let _render = RenderTimeTracer::new("blockpage");
    let height: BlockHeight = req.param("height").unwrap().parse().map_err(to_badreq)?;
    let last_snap = req.state().snapshot().await.map_err(to_badgateway)?;
    let block = last_snap
        .get_older(height)
        .await
        .map_err(to_badgateway)?
        .current_block()
        .await?;
    let reward_coin = last_snap
        .get_older(height)
        .await
        .map_err(to_badgateway)?
        .get_coin(CoinID::proposer_reward(height))
        .await
        .map_err(to_badgateway)?;
    let reward_amount = reward_coin.map(|v| v.coin_data.value).unwrap_or_default();

    let mut body: tide::Body = BlockTemplate {
        testnet: req.state().netid() == NetID::Testnet,
        header: block.header,
        txcount: block.transactions.len(),
        txweight: block.transactions.iter().map(|v| v.weight()).sum(),
        txhashes: block
            .transactions
            .iter()
            .map(|v| (v.hash_nosigs(), v.weight()))
            .collect(),
        fee_multiplier: block.header.fee_multiplier as f64 / 65536.0,
        _reward_amount: MicroUnit(reward_amount.0, "MEL".into()),
        total_fees: MicroUnit(
            block.transactions.iter().map(|v| v.fee.0).sum(),
            "MEL".into(),
        ),
        fee_pool: MicroUnit(block.header.fee_pool.0, "MEL".into()),
        tooltips: serde_json::from_str(include_str!("../tooltips.json")).unwrap(),
    }
    .render()
    .unwrap()
    .into();
    body.set_mime("text/html");
    Ok(body)
}
