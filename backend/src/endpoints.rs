use std::collections::HashMap;
use std::collections::{BTreeMap, BTreeSet};
use std::convert::{Infallible, TryInto};
use std::fmt::Display;

use crate::{globals::CLIENT, raw::*};
use anyhow::Context;
use futures_util::Future;
use rweb::*;

use serde::ser::SerializeTupleStruct;

use serde::Serialize;
use smol::Task;
use themelio_stf::melvm::covenant_weight_from_bytes;
use themelio_structs::*;
use tracing::{debug, info};

type DynReply = Result<Box<dyn warp::Reply>, Infallible>;

// the reusable helper function
async fn generic_fallible<R: warp::Reply + 'static>(
    f: impl Future<Output = anyhow::Result<R>>,
) -> DynReply {
    match f.await {
        Ok(res) => Ok(Box::new(res)),
        Err(err) => {
            let mut map = HashMap::new();
            let err_string = err.to_string();
            info!("{err_string}");
            map.insert("error", err.to_string());
            Ok(Box::new(rweb::reply::with_status(
                rweb::reply::json(&map),
                rweb::hyper::StatusCode::INTERNAL_SERVER_ERROR,
            )))
        }
    }
}

/// Helper function for JSON
async fn generic_fallible_json<R: Serialize>(
    data: impl Future<Output = anyhow::Result<R>>,
) -> DynReply {
    generic_fallible(async {
        let json = rweb::reply::json(&data.await?);
        Ok(json)
    })
    .await
}

/// Helper function for JSON that returns a 404 for None.
async fn generic_fallible_json_option<R: Serialize>(
    data: impl Future<Output = anyhow::Result<Option<R>>>,
) -> DynReply {
    generic_fallible(async {
        let r: Box<dyn warp::Reply> = match data.await? {
            Some(data) => {
                let json = rweb::reply::json(&data);
                Box::new(json)
            }
            None => Box::new(rweb::reply::with_status(
                rweb::reply::reply(),
                rweb::hyper::StatusCode::NOT_FOUND,
            )),
        };
        Ok(r)
    })
    .await
}

#[get("/raw/overview")]
pub async fn overview() -> DynReply {
    generic_fallible_json(async move {
        let overview = get_overview(CLIENT.to_owned(), None).await?;
        let height = overview.recent_blocks[0].header.height;
        let mut o = overview.clone();
        o.recent_blocks = vec![];
        let overview2 = serde_json::to_string_pretty(&o)?;
        debug!("Found Height: {height}");
        println!("{overview2:?}");
        anyhow::Ok(overview)
    })
    .await
}

#[get("/raw/latest")]
pub async fn latest() -> DynReply {
    generic_fallible_json(get_latest_header(CLIENT.to_owned())).await
}

#[get("/raw/blocks/{height}/transactions/{txhash}")]
pub async fn transaction(height: u64, txhash: String) -> DynReply {
    generic_fallible_json_option(get_transaction(&CLIENT.to_owned(), height, txhash)).await
}

#[get("/raw/blocks/{height}/coins/{coinid}")]
pub async fn coins(height: u64, coinid: String) -> DynReply {
    generic_fallible_json_option(get_coin(CLIENT.to_owned(), height, coinid)).await
}

#[get("/raw/blocks/{height}/full")]
pub async fn block_full(height: u64) -> DynReply {
    generic_fallible_json(get_full_block(CLIENT.to_owned(), height)).await
}

#[get("/raw/blocks/{height}/summary")]
pub async fn block_summary(height: u64) -> DynReply {
    generic_fallible_json(get_block_summary(CLIENT.to_owned(), height)).await
}
#[get("/raw/blocks/{height}/pools/{left}/{right}")]
pub async fn pool(height: u64, left: Denom, right: Denom) -> DynReply {
    generic_fallible_json_option(get_pool(CLIENT.to_owned(), height, left, right)).await
}

#[get("/raw/pooldata/{denom_left}/{denom_right}/{lowerblock}/{upperblock}")]
pub async fn pooldata(
    denom_left: Denom,
    denom_right: Denom,
    lowerblock: u64,
    upperblock: u64,
) -> DynReply {
    generic_fallible_json(get_pooldata_range(
        CLIENT.to_owned(),
        denom_left,
        denom_right,
        lowerblock,
        upperblock,
    ))
    .await
}

#[derive(Debug)]
struct MicroUnit(u128, Denom);

impl Display for MicroUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{:06} {}",
            self.0 / MICRO_CONVERTER,
            self.0 % MICRO_CONVERTER,
            self.1,
        )
    }
}

impl Serialize for MicroUnit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_tuple_struct("MicroUnit", 2)?;
        let value = &format!(
            "{}.{:06}",
            self.0 / MICRO_CONVERTER,
            self.0 % MICRO_CONVERTER
        );
        state.serialize_field(value)?;
        state.serialize_field(&self.1.to_string())?;
        state.end()
    }
}

type Inputs = Vec<(usize, CoinID, CoinDataHeight, MicroUnit, String, String)>;
type Outputs = Vec<(usize, CoinData, MicroUnit, String, String)>;
#[derive(Serialize, Debug)]
struct TransactionTemplate {
    testnet: bool,
    txhash: TxHash,
    txhash_abbr: String,
    height: u64,
    transaction: Transaction,
    kind: String,
    inputs_with_cdh: Inputs,
    outputs: Outputs,
    fee: MicroUnit,
    base_fee: MicroUnit,
    tips: MicroUnit,
    net_loss: BTreeMap<String, Vec<MicroUnit>>,
    net_gain: BTreeMap<String, Vec<MicroUnit>>,
    gross_gain: Vec<MicroUnit>,
    weight: u128,
}

#[get("/raw/blocks/{height}/{txhash}")]
pub async fn transaction_page(height: u64, txhash: String) -> DynReply {
    generic_fallible_json_option(async move {
        let client = CLIENT.to_owned();
        let txhash: TxHash = TxHash(txhash.parse()?);
        let snap = client.older_snapshot(height).await?;
        let transaction = if let Some(tx) = snap.get_transaction(txhash).await? {
            tx
        } else {
            return Ok(None);
        };
        let tmapping: BTreeMap<CoinID, Task<anyhow::Result<CoinDataHeight>>> = transaction
            .inputs
            .iter()
            .map(|cid| {
                let cid = *cid;
                let snap = snap.clone();
                (
                    cid,
                    smolscale::spawn(async move {
                        snap.get_coin_spent_here(cid)
                            .await?
                            .context("Error getting")
                    }),
                )
            })
            .collect();
        let mut coin_map: BTreeMap<CoinID, CoinDataHeight> = BTreeMap::new();
        for (i, (cid, task)) in tmapping.into_iter().enumerate() {
            debug!("resolving input {} for {}", i, txhash);
            coin_map.insert(cid, task.await?);
        }

        // now that we have the transaction, we can construct the info.
        let denoms: BTreeSet<_> = transaction.outputs.iter().map(|v| v.denom).collect();
        let mut net_loss: BTreeMap<String, Vec<MicroUnit>> = BTreeMap::new();
        let mut net_gain: BTreeMap<String, Vec<MicroUnit>> = BTreeMap::new();

        for denom in denoms {
            let mut balance: BTreeMap<Address, i128> = BTreeMap::new();
            // we add to the balance
            for output in transaction.outputs.iter() {
                if output.denom == denom {
                    let new_balance = balance
                        .get(&output.covhash)
                        .cloned()
                        .unwrap_or_default()
                        .checked_add(output.value.0.try_into()?)
                        .context("cannot add")?;
                    balance.insert(output.covhash, new_balance);
                }
            }
            // we subtract from the balance
            for input in transaction.inputs.iter() {
                debug!("getting input {} of {}", input, transaction.hash_nosigs());
                let cdh = coin_map[input].clone();
                if cdh.coin_data.denom == denom {
                    let new_balance = balance
                        .get(&cdh.coin_data.covhash)
                        .cloned()
                        .unwrap_or_default()
                        .checked_sub(cdh.coin_data.value.0.try_into()?)
                        .context("cannot add")?;
                    balance.insert(cdh.coin_data.covhash, new_balance);
                }
            }
            // we update net loss/gain
            for (addr, balance) in balance {
                if balance < 0 {
                    net_loss
                        .entry(addr.0.to_addr())
                        .or_default()
                        .push(MicroUnit((-balance) as u128, denom));
                } else if balance > 0 {
                    net_gain
                        .entry(addr.0.to_addr())
                        .or_default()
                        .push(MicroUnit(balance as u128, denom));
                }
            }
        }

        let fee = transaction.fee;
        let fee_mult = snap
            .get_older((height - 1).into())
            .await?
            .current_header()
            .fee_multiplier;
        let base_fee = transaction
            .base_fee(fee_mult, 0, covenant_weight_from_bytes)
            .0;
        let tips = fee.0.saturating_sub(base_fee);

        let mut inputs_with_cdh: Inputs = vec![];
        // we subtract from the balance
        for (index, input) in transaction.inputs.iter().copied().enumerate() {
            debug!("rendering input {} of {}", index, transaction.hash_nosigs());
            let cdh = coin_map[&input].clone();
            inputs_with_cdh.push((
                index,
                input,
                cdh.clone(),
                MicroUnit(cdh.coin_data.value.into(), cdh.coin_data.denom),
                cdh.coin_data.additional_data_hex(),
                cdh.coin_data.covhash.0.to_addr(),
            ));
        }

        let body = TransactionTemplate {
            testnet: client.netid() == NetID::Testnet,
            txhash,
            txhash_abbr: hex::encode(&txhash.0[..5]),
            height,
            transaction: transaction.clone(),
            net_loss,
            inputs_with_cdh,
            net_gain,
            outputs: transaction
                .outputs
                .iter()
                .enumerate()
                .map(|(i, cd)| {
                    (
                        i,
                        cd.clone(),
                        MicroUnit(cd.value.0, cd.denom),
                        cd.additional_data_hex(),
                        cd.covhash.0.to_addr(),
                    )
                })
                .collect(),
            fee: MicroUnit(fee.0, Denom::Mel),
            base_fee: MicroUnit(base_fee, Denom::Mel),
            tips: MicroUnit(tips, Denom::Mel),
            gross_gain: transaction
                .total_outputs()
                .iter()
                .map(|(denom, val)| MicroUnit(val.0, *denom))
                .collect(),
            weight: transaction.weight(themelio_stf::melvm::covenant_weight_from_bytes),
            kind: format!("{}", transaction.kind),
        };

        Ok(Some(body))
    })
    .await
}
