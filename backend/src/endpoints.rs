use std::collections::HashMap;
use std::convert::Infallible;

use crate::raw::*;
use futures_util::Future;
use rweb::*;
use serde::{Deserialize, Serialize};

type DynReply = Result<Box<dyn warp::Reply>, Infallible>;

// the reusable helper function
async fn generic_fallible<R: warp::Reply + 'static>(
    f: impl Future<Output = anyhow::Result<R>>,
) -> DynReply {
    match f.await {
        Ok(res) => Ok(Box::new(res)),
        Err(err) => {
            let mut map = HashMap::new();
            map.insert("error", err.to_string());
            Ok(Box::new(rweb::reply::with_status(
                rweb::reply::json(&map),
                rweb::hyper::StatusCode::INTERNAL_SERVER_ERROR,
            )))
        }
    }
}

async fn generic_fallible_json<R: Serialize>(
    data: impl Future<Output = anyhow::Result<R>>,
) -> DynReply {
    generic_fallible(async {
        let json = rweb::reply::json(&data.await?);
        Ok(json)
    })
    .await
}
#[derive(Deserialize)]
struct LoginForm {
    id: String,
    password: String,
}

#[get("/raw/overview")]
pub async fn overview() -> DynReply {
    generic_fallible_json(get_overview(None)).await
}

#[get("/raw/latest")]
pub async fn latest() -> DynReply {
    generic_fallible_json(get_latest()).await
}

// #[get("/raw/blocks/{height}/transactions/{txhash}")]
// pub async fn get_transaction(#[data] client: ValClient, height: String, txhash: String) -> DynReply {
//     generic_fallible_json(get_latest(client)).await
// }

// // #[get("/raw/blocks/{height}")]
// // pub async fn get_block_summary(#[data] client: ValClient) -> DynReply {
// //     generic_fallible_json(get_block_summary(client)).await
// // }

// #[get("/raw/blocks/{height}/summary")]
// pub async fn get_block_summary(#[data] client: ValClient) -> DynReply {
//     generic_fallible_json(get_block_summary(client)).await
// }
// #[get("/raw/blocks/{height}/full")]
// pub async fn get_full_block(#[data] client: ValClient) -> DynReply {
//     generic_fallible_json(get_full_block(client)).await
// }
// #[get("/raw/blocks/{height}/coins/{coinid}")]
// pub async fn get_coin(#[data] client: ValClient) -> DynReply {
//     generic_fallible_json(get_coin(client)).await
// }
// #[get("/raw/blocks/{height}/pools/{denom}")]
// pub async fn get_pool(#[data] client: ValClient) -> DynReply {
//     generic_fallible_json(get_pool(client)).await
// }
// #[get("/raw/pool-data-batch/{lowerblock}")]
// pub async fn get_pooldata(#[data] client: ValClient) -> DynReply {
//     generic_fallible_json(get_pooldata(client)).await
// }
// #[get("/raw/pooldata/{denom_left}/{denom_right}/{lowerblock}/{upperblock}")]
// pub async fn get_pooldata_range(#[data] client: ValClient) -> DynReply {
//     generic_fallible_json(get_pooldata_range(client)).await
// }
