use endpoints::*;
use rweb::Filter;
use tracing_subscriber::{util::SubscriberInitExt, EnvFilter};

use crate::globals::CMD_ARGS;
mod endpoints;
mod globals;
mod raw;
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
    let log_conf = std::env::var("RUST_LOG").unwrap_or_else(|_| "melscan=debug,warn".into());
    std::env::set_var("RUST_LOG", log_conf);
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_ansi(false)
        .finish()
        .init();

    let port = 13000;
    println!("Serving on port: {port}");

    let routes = routes![
        overview,
        latest,
        transaction,
        coins,
        block_full,
        block_summary,
        pool,
        pooldata
    ];
    rweb::serve(routes).run(CMD_ARGS.listen).await;

    Ok(())
}
