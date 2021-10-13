#![allow(unused_imports)]
use std::{convert::TryInto, net::SocketAddr};

use std::fmt::Debug;
use structopt::StructOpt;
use themelio_nodeprot::{TrustedHeight, ValClient};
use themelio_stf::NetID;
use tide::StatusCode;
use tmelcrypt::HashVal;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;
mod html;
mod raw;
mod utils;

fn main() -> anyhow::Result<()> {
    smol::block_on(main_inner())
}

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
}

#[tracing::instrument]
async fn main_inner() -> anyhow::Result<()> {
    let log_conf = std::env::var("RUST_LOG")
        .unwrap_or_else(|_| "melscan=debug,themelio_nodeprot=debug,warn".into());
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
    let mut app = tide::with_state(client);
    // Rendered paths
    app.at("/").get(html::get_homepage);
    app.at("/blocks/:height").get(html::get_blockpage);
    app.at("/pools/:denom").get(html::get_poolpage);
    app.at("/blocks/:height/:txhash").get(html::get_txpage);
    // Raw paths
    app.at("/raw/latest").get(raw::get_latest);
    app.at("/raw/blocks/:height").get(raw::get_header);
    app.at("/raw/blocks/:height/summary").get(raw::get_block_summary);
    app.at("/raw/blocks/:height/full").get(raw::get_full_block);
    app.at("/raw/blocks/:height/transactions/:txhash")
        .get(raw::get_transaction);
    app.at("/raw/blocks/:height/coins/:coinid")
        .get(raw::get_coin);
    app.at("/raw/blocks/:height/pools/:denom")
        .get(raw::get_pool);
    app.with(tide::utils::After(|mut res: tide::Response| async move {
        if let Some(err) = res.error() {
            // put the error string in the response
            let err_str = format!("ERROR: {:?}", err);
            log::warn!("{}", err_str);
            res.set_body(err_str);
        }
        Ok(res)
    }));
    tracing::info!("Starting REST endpoint at {}", args.listen);
    app.listen(args.listen).await?;
    Ok(())
}

fn to_badreq<E: Into<anyhow::Error> + Send + 'static + Sync + Debug>(e: E) -> tide::Error {
    tide::Error::new(StatusCode::BadRequest, e)
}

fn to_badgateway<E: Into<anyhow::Error> + Send + 'static + Sync + Debug>(e: E) -> tide::Error {
    tide::Error::new(StatusCode::BadGateway, e)
}

fn notfound() -> tide::Error {
    tide::Error::new(StatusCode::NotFound, anyhow::anyhow!("not found"))
}
