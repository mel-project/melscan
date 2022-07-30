use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::{BTreeMap, BTreeSet};
use std::convert::{Infallible, TryInto};
use std::fmt::Display;
use std::io::Cursor;
use std::num::ParseIntError;
use std::ops::Range;
use std::str::FromStr;

use anyhow::Context;
use chrono::Utc;
use dashmap::DashMap;

use futures_util::future::join_all;
use futures_util::Future;
use num_traits::ToPrimitive;
use once_cell::sync::Lazy;
use rweb::*;

use serde::{ser::SerializeTupleStruct, Deserialize};

use serde::Serialize;
use smol::Task;
use themelio_stf::melvm::covenant_weight_from_bytes;
use themelio_structs::*;
use tmelcrypt::{HashVal, Hashable};
use tracing::{debug, info};

use crate::{
    globals::{BACKEND, CLIENT},
    graphs::{datetime_to_height, graph_range},
};
use themelio_stf::melvm::opcode;

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
        let overview = BACKEND.get_overview(None).await?;
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
    generic_fallible_json(BACKEND.get_latest_header()).await
}

#[get("/raw/search/transaction/{txhash}")]
pub async fn search_transaction(txhash: TxHash) -> DynReply {
    generic_fallible_json_option(BACKEND.search_transaction(txhash)).await
}

#[get("/raw/search/block/{blkhash}")]
pub async fn search_block(blkhash: HashVal) -> DynReply {
    generic_fallible_json_option(BACKEND.search_block(blkhash)).await
}

#[get("/raw/blocks/{height}/transactions/{txhash}")]
pub async fn transaction(height: BlockHeight, txhash: TxHash) -> DynReply {
    generic_fallible_json_option(BACKEND.get_transaction_at_height(height, txhash)).await
}

#[get("/debug/{height}/{txhash}/{index}")]
pub async fn debug_spent_coin(height: BlockHeight, txhash: TxHash, index: u8) -> DynReply {
    let closure = async move {
        let coinid = CoinID { txhash, index };
        let coin = BACKEND
            .get_coin_at_height(height, coinid)
            .await?
            .context("No coin with this shape")?;

        Ok(Some(is_spent(height.0, &coinid).await?))
    };
    generic_fallible_json_option(closure).await
}

// if the coin is found at the current height it is not spent; assumes coin existed
pub async fn is_spent(height: u64, coinid: &CoinID) -> anyhow::Result<bool> {
    let coin = BACKEND
        .get_coin_at_height(BlockHeight(height), *coinid)
        .await?;

    Ok(coin.is_none())
}

pub async fn find_spending_height(
    coin: CoinID,
    height_range: Range<u64>,
) -> anyhow::Result<Option<BlockHeight>> {
    let height_range: Vec<u64> = height_range.collect();

    if height_range.is_empty() {
        return Ok(None);
    }

    let index = {
        let mut max_index = height_range.last().unwrap().to_owned(); // will never be None since the range is not empty
        let mut min_index = height_range.first().unwrap().to_owned();
        while max_index - min_index > 1 {
            // println!("Bounding heights: {} {}", min_index, max_index);

            let check_index = (max_index - min_index) / 2 + min_index;
            let spent = is_spent(check_index, &coin).await?;

            // println!("Checking: {} Spent: {}", check_index, spent);

            match spent {
                true => max_index = check_index,
                false => min_index = check_index,
            }
        }
        max_index
    };

    let spend_edge: Vec<BlockHeight> = join_all([index - 1, index].map(|index| {
        async move {
            let spend_height = BlockHeight(index);
            let spend_coin_data = BACKEND.get_coin_at_height(spend_height, coin).await?; // would be nice to replace with a more lightweight function

            // println!("Spent here? {} {:?}", spend_height, spend_coin_data);
            anyhow::Ok(match spend_coin_data {
                Some(_) => None,
                None => Some(spend_height),
            })
        }
    }))
    .await
    .into_iter()
    .flatten()
    .flatten()
    .collect();

    match spend_edge.len() {
        1 => Ok(Some(spend_edge[0])),
        0 => Ok(None),
        // the binary search above is convergent and always terminates
        // `spend_edge_result` always has at most 2 values and values are never added
        _ => unreachable!(),
    }
}

pub async fn find_spending_transaction(
    block: Block,
    coinid: CoinID,
) -> anyhow::Result<Option<Transaction>> {
    let tx = block
        .transactions
        .iter()
        .find(|&tx| tx.inputs.clone().into_iter().any(|input| coinid == input))
        .cloned();
    Ok(tx)
}
#[derive(Clone, Debug, Serialize)]
struct CoinLocation {
    coinid: CoinID,
    txhash: TxHash,
    height: BlockHeight,
}

pub async fn find_spend_within_range(
    coinid: CoinID,
    height_range: Range<u64>,
) -> anyhow::Result<Option<CoinLocation>> {
    let range = height_range;
    let spend_height = match find_spending_height(coinid, range).await? {
        Some(spend) => spend,
        None => return Ok(None),
    };

    let snapshot = BACKEND.client.older_snapshot(spend_height).await?;
    let block = snapshot.current_block().await?;

    let spend_tx = find_spending_transaction(block, coinid).await?;
    let spend_txhash = spend_tx
        .context("Unexpected Failure: couldn't find spending transaction in spending block")?
        .hash_nosigs();
    println!("{coinid}{spend_height}");

    Ok(Some(CoinLocation {
        coinid,
        txhash: spend_txhash,
        height: spend_height,
    }))
}
#[get("/raw/blocks/{height}/{txhash}/spends")]
pub async fn transaction_spenders(height: BlockHeight, txhash: TxHash) -> DynReply {
    let closure = async move {
        let tx = BACKEND
            .get_transaction_at_height(height, txhash)
            .await?
            .context("No transaction found")?;
        let chain_height = BACKEND.get_latest_header().await?.height;
        let height_range = height.0..chain_height.0;

        let output_len = tx.outputs.len() as u8;
        // let outputs_range: Range<u8> = 0..1;
        let outputs_range: Range<u8> = 0..output_len;

        let coin_spends: Vec<Option<CoinLocation>> = join_all(
            outputs_range
                .map(|index| CoinID { txhash, index })
                .map(|coinid| find_spend_within_range(coinid, height_range.clone())),
        )
        .await
        .into_iter()
        .flatten()
        .collect();
        Ok(Some(coin_spends))
    };
    generic_fallible_json_option(closure).await
}

#[get("/raw/blocks/{height}/coins/{coinid}")]
pub async fn coins(height: BlockHeight, coinid: CoinID) -> DynReply {
    generic_fallible_json_option(BACKEND.get_coin_at_height(height, coinid)).await
}

#[get("/raw/blocks/{height}/full")]
pub async fn block_full(height: BlockHeight) -> DynReply {
    generic_fallible_json(BACKEND.get_block(height)).await
}

#[get("/raw/blocks/{height}/summary")]
pub async fn block_summary(height: BlockHeight) -> DynReply {
    generic_fallible_json(BACKEND.get_block_summary(height)).await
}

#[get("/raw/address/{address}")]
pub async fn address_summary(address: Address) -> DynReply {
    generic_fallible_json(BACKEND.get_address_summary(address)).await
}

#[get("/raw/leaderboard/{denom}")]
pub async fn leaderboard(denom: Denom) -> DynReply {
    generic_fallible_json(BACKEND.get_leaderboard(denom)).await
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
struct GraphQuery {
    id: GraphId,
    start: Option<chrono::DateTime<Utc>>,
    end: Option<chrono::DateTime<Utc>>,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, PartialOrd, Hash, Eq)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum GraphId {
    PoolPrice {
        #[serde(with = "serde_with::rust::display_fromstr")]
        from: Denom,
        #[serde(with = "serde_with::rust::display_fromstr")]
        to: Denom,
    },

    PoolLiquidity {
        #[serde(with = "serde_with::rust::display_fromstr")]
        from: Denom,
        #[serde(with = "serde_with::rust::display_fromstr")]
        to: Denom,
    },

    CoinSupply {
        #[serde(with = "serde_with::rust::display_fromstr")]
        denom: Denom,
    },
}

#[post("/raw/graph")]
pub async fn graph(#[json] qs: GraphQuery) -> DynReply {
    generic_fallible_json_option(async move {
        let snapshot = CLIENT.snapshot().await?;
        let start = qs
            .start
            .map(datetime_to_height)
            .unwrap_or(BlockHeight(1))
            .max(BlockHeight(1));
        let end = qs
            .end
            .map(datetime_to_height)
            .unwrap_or_else(|| snapshot.current_header().height);
        static GRAPH_CACHE: Lazy<DashMap<(GraphId, BlockHeight), f64>> = Lazy::new(DashMap::new);
        // figure out *which* graph to draw
        let load_cache = move |height| GRAPH_CACHE.get(&(qs.id, height)).map(|s| *s);
        let store_cache = move |height, res| {
            GRAPH_CACHE.insert((qs.id, height), res);
        };
        Ok(Some(match qs.id {
            GraphId::PoolPrice { from, to } => {
                graph_range(
                    start,
                    end,
                    1000,
                    move |height| async move {
                        let snap = CLIENT.older_snapshot(height).await?;
                        let pool_key = PoolKey::new(from, to);
                        let pool_info = snap.get_pool(pool_key).await?;
                        if let Some(pool_info) = pool_info {
                            let ratio = pool_info.implied_price().to_f64().unwrap_or(f64::NAN);
                            if pool_key.left == from {
                                Ok(1.0 / ratio)
                            } else {
                                Ok(ratio)
                            }
                        } else {
                            Ok(f64::NAN)
                        }
                    },
                    load_cache,
                    store_cache,
                )
                .await?
            }
            GraphId::PoolLiquidity { from, to } => {
                graph_range(
                    start,
                    end,
                    1000,
                    move |height| async move {
                        let snap = CLIENT.older_snapshot(height).await?;
                        let pool_key = PoolKey::new(from, to);
                        let pool_info = snap.get_pool(pool_key).await?;
                        if let Some(pool_info) = pool_info {
                            Ok((pool_info.liq_constant() as f64).sqrt() / 1_000_000.0)
                        } else {
                            Ok(f64::NAN)
                        }
                    },
                    load_cache,
                    store_cache,
                )
                .await?
            }
            GraphId::CoinSupply { denom } => {
                graph_range(
                    start,
                    end.min(BACKEND.indexed_highest()),
                    300,
                    move |height| async move {
                        let v = BACKEND
                            .get_coin_supply(height, denom)
                            .await?
                            .map(|c| (c.0 as f64) / 1_000_000.0)
                            .unwrap_or(f64::NAN);
                        Ok(v)
                    },
                    load_cache,
                    store_cache,
                )
                .await?
            }
        }))
    })
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

type OpCodeString = String;
type OpCodeStrings = Vec<OpCodeString>;
type Inputs = Vec<(usize, CoinID, CoinDataHeight, MicroUnit, String, String)>;
type Outputs = Vec<(usize, CoinData, MicroUnit, String, String)>;
#[derive(Serialize, Debug)]
struct TransactionTemplate {
    testnet: bool,
    txhash: TxHash,
    txhash_abbr: String,
    height: BlockHeight,
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
    covenants: Vec<(String, OpCodeStrings)>,
}

fn decode_all_ops(covenant: Vec<u8>) -> anyhow::Result<OpCodeStrings> {
    let mut opcode_cursor: Cursor<Vec<u8>> = Cursor::new(covenant);
    let mut ops: Vec<OpCodeString> = vec![];
    while opcode_cursor.has_remaining() {
        let opcode = opcode::OpCode::decode(&mut opcode_cursor)?;
        let fmt = format!("{opcode:?}")
            .replace('(', " ")
            .replace(')', "")
            .replace(',', " ");
        ops.push(fmt);
    }
    Ok(ops)
}
pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

#[get("/raw/blocks/{height}/{txhash}")]
pub async fn transaction_page(height: BlockHeight, txhash: TxHash) -> DynReply {
    generic_fallible_json_option(async move {
        let client = CLIENT.to_owned();
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
            .get_older((height.0 - 1).into())
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
        let covenants = transaction
            .clone()
            .covenants
            .into_iter()
            .map(|cov| anyhow::Ok((Address(cov.hash()).to_string(), decode_all_ops(cov)?)))
            .collect::<Result<_, _>>()?;

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
            covenants,
        };

        Ok(Some(body))
    })
    .await
}
