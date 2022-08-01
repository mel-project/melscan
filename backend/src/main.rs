use endpoints::*;
use rweb::{hyper::Method, Filter};
use tracing_subscriber::{util::SubscriberInitExt, EnvFilter};

use crate::globals::CMD_ARGS;
mod backend;
mod crawl;
mod endpoints;
mod globals;
mod graphs;
mod utils;

#[macro_export]
macro_rules! routes {
    ( $s:expr ) => {
        // This is used when you use routes! with a single route without any data; I.e routes!(ping)
        $s()
    };
    ( $inject:expr; $s:expr ) => {
        /// This is used when you use routes! with a single route and want to pass some data to it; I.e routes!(db_connection; get_user)
        $s($inject)
    };
    ( $s:expr, $( $x:expr ),* ) => {
        // This is used when you use routes! with multiple routes without any data: I.e routes!(ping, get_users, get_users)
            $s()
            $(
                .or($x())
            )*
    };
    ( $inect:expr; $s:expr, $( $x:expr ),* ) => {
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
    let log_conf = std::env::var("RUST_LOG").unwrap_or_else(|_| "melscan=debug,warn,info".into());
    std::env::set_var("RUST_LOG", log_conf);
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_ansi(false)
        .finish()
        .init();

    log::info!("listening on {}", CMD_ARGS.listen);

    let routes = routes![
        overview,
        latest,
        transaction,
        coins,
        block_full,
        block_summary,
        address_summary,
        search_transaction,
        search_block,
        leaderboard,
        transaction_page,
        graph,
        transaction_crawl
    ];
    let cors = warp::cors()
        .allow_any_origin()
        // .allow_credentials(true)
        .allow_method(Method::GET)
        .allow_method(Method::POST)
        .allow_method(Method::OPTIONS)
        .allow_header("content-type");
    rweb::serve(routes.with(cors).with(warp::trace(|info| {
        // Create a span using tracing macros
        tracing::info_span!(
            "request",
            method = %info.method(),
            path = %info.path(),
        )
    })))
    .run(CMD_ARGS.listen)
    .await;

    Ok(())
}
