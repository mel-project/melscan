use std::convert::Infallible;
use std::{collections::HashMap, str::FromStr};

use crate::{globals::CLIENT, raw::*};
use futures_util::Future;
use rweb::*;
use serde::{Deserialize, Serialize};
use tracing::info;

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

#[derive(Debug, Schema, Serialize, Deserialize)]
struct Denom(themelio_structs::Denom);

impl FromStr for Denom {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Denom(themelio_structs::Denom::from_str(s)?))
    }
}


#[get("/raw/overview")]
pub async fn overview() -> DynReply {
    let overview = get_overview(CLIENT.to_owned(), None);
    generic_fallible_json(overview).await
}

#[get("/raw/latest")]
pub async fn latest() -> DynReply {
    generic_fallible_json(get_latest_header(CLIENT.to_owned())).await
}

#[get("/raw/blocks/{height}/transactions/{txhash}")]
pub async fn transaction(height: u64, txhash: String) -> DynReply {
    generic_fallible_json_option(get_transaction(CLIENT.to_owned(), height, txhash)).await
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
    generic_fallible_json_option(get_pool(CLIENT.to_owned(), height, left.0, right.0)).await
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
        denom_left.0,
        denom_right.0,
        lowerblock,
        upperblock,
    ))
    .await
}
