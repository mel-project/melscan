use std::collections::HashMap;
use std::convert::Infallible;

use crate::utils::*;
use crate::{State};
use async_trait::async_trait;
use futures_util::Future;
use rweb::*;
use themelio_nodeprot::ValClient;
use serde::{Serialize, Deserialize};
use crate::raw::*;



macro_rules! routes {
    ( $s:expr ) => {
        // This is used when you use routes! with a single route without any data; I.e routes!(ping)
        $s()
    };
    ( $inject:expr; $s:expr ) => {
        // This is used when you use routes! with a single route and want to pass some data to it; I.e routes!(db_connection; get_user)
        $s($inject)
    };
    ( $s:expr, $( $x:expr ),* ) => {
        // This is used when you use routes! with multiple routes without any data: I.e routes!(ping, get_users, get_users)
            $s()
            $(
                .or($x())
            )*
    };
    ( $inject:expr; $s:expr, $( $x:expr ),* ) => {
        // This is used when you use routes! with multiple routes and want to pass some data to it: I.e routes!(db_connection; ping, get_users, get_users)
            $s(inject)
            $(
                .or($x($inject))
            )*
    };
}



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
    data: impl Future<Output= anyhow::Result<R>>
) -> DynReply{
    generic_fallible(async {
        let json = rweb::reply::json(&data.await?);
        Ok(json)
    }).await
}
#[derive(Deserialize)]
struct LoginForm {
    id: String,
    password: String,
}

#[get("/{height}/{txhash}")]
fn no_data_and_paras(#[data] client: ValClient, height: u64, txhash: String) ->
String {     
    "Hello".into()
}


#[get("/raw/overview")]
pub async fn overview(#[data] client: ValClient) -> DynReply {
    generic_fallible_json(get_overview(client, None)).await
} 

#[get("/raw/latest")]
pub async fn latest(#[data] client: ValClient) -> DynReply {
    generic_fallible_json(get_latest(client)).await
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