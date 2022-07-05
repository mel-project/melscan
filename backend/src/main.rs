use std::{net::SocketAddr, path::PathBuf, sync::Arc};

use dashmap::DashMap;
use std::fmt::Debug;
use structopt::StructOpt;
use themelio_nodeprot::ValClient;
use themelio_structs::NetID;

use tracing_subscriber::{util::SubscriberInitExt, EnvFilter};

use crate::indexer::Indexer;
mod indexer;
mod raw;
mod utils;
mod endpoints;
// fn main() -> anyhow::Result<()> {
//     smol::block_on(main_inner())
// }

#[derive(StructOpt)]
pub struct Args {
    #[structopt(long)]
    /// Where to listen for incoming REST API calls
    listen: SocketAddr,

    #[structopt(long)]
    /// A full node to connect to
    connect: SocketAddr,

    #[structopt(long)]
    /// Whether or not the block explorer is connected to a testnet node.
    testnet: bool,

    #[structopt(long)]
    /// Path to a "full" index file. If this is present, will act as a "full node" to pull huge amounts of info from the blockchain.
    index_path: Option<PathBuf>,
}

#[derive(Clone)]
pub struct State {
    // raw_pooldata_cache: Arc<DashMap<raw::PoolInfoKey, Option<html::PoolDataItem>>>,
    val_client: ValClient,
}

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



#[tracing::instrument]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let log_conf = std::env::var("RUST_LOG").unwrap_or_else(|_| "melscan=debug,warn".into());
    std::env::set_var("RUST_LOG", log_conf);
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_ansi(false)
        .finish()
        .init();

    let args = Args::from_args();
    let client = ValClient::new(
        if args.testnet {
            NetID::Testnet
        } else {
            NetID::Mainnet
        },
        args.connect,
    );

    // TODO read this from an argument or a special crate
    if args.testnet {
        client.trust(themelio_bootstrap::checkpoint_height(NetID::Testnet).unwrap());
    } else {
        client.trust(themelio_bootstrap::checkpoint_height(NetID::Mainnet).unwrap());
    }

    let _indexer = args
        .index_path
        .clone()
        .map(|path| Indexer::new(&path, client.clone()));

    let state = State {
        // raw_pooldata_cache: DashMap::new().into(),
        val_client: client,
    };

    // let mut app = tide::with_state(state.clone());
    // let cors = CorsMiddleware::new()
    //     .allow_methods("GET, POST, OPTIONS".parse::<HeaderValue>().unwrap())
    //     .allow_origin(Origin::from("*"))
    //     .allow_credentials(false);
    // // Rendered paths
    // app.at("/robots.txt").get(|req| async {
    //     let t = include_str!("robots.txt");
    //     Ok(Body::from_string(t.to_string()))
    // });
    // app.at("/blocks/:height/:txhash").get(html::get_txpage);
    // // Raw paths
    // app.at("/raw/overview/:height").get(raw::get_overview);
    // app.at("/raw/overview").get(raw::get_overview);
    // app.at("/raw/latest").get(raw::get_latest);
    // app.at("/raw/blocks/:height").get(raw::get_block_summary);
    // app.at("/raw/blocks/:height/summary")
    //     .get(raw::get_block_summary);
    // app.at("/raw/blocks/:height/full").get(raw::get_full_block);
    // app.at("/raw/blocks/:height/transactions/:txhash")
    //     .get(raw::get_transaction);
    // app.at("/raw/blocks/:height/coins/:coinid")
    //     .get(raw::get_coin);
    // app.at("/raw/blocks/:height/pools/:denom")
    //     .get(raw::get_pool);
    // // app.at("/raw/pool-data-batch/:lowerblock").get(raw::get_pooldata);
    // app.at("/raw/pooldata/:denom_left/:denom_right/:lowerblock/:upperblock")
    //     .get(raw::get_pooldata_range);
    // app.with(tide::utils::After(|mut res: tide::Response| async move {
    //     if let Some(err) = res.error() {
    //         // put the error string in the response
    //         let err_str = format!("ERROR: {:?}", err);
    //         log::warn!("{}", err_str);
    //         res.set_body(err_str);
    //     }
    //     Ok(res)
    // }))
    // .with(cors);let routes = 
    // tracing::info!("Starting REST endpoint at {}", args.listen);
    // app.listen(args.listen).await?;
    
    let client = state.val_client.clone();
    let routes = routes![client.clone(); endpoints::get_overview_rweb];
    rweb::serve(routes).run(([127, 0, 0, 1], 13000)).await;


    Ok(())
}



