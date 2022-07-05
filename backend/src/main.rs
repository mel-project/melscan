use tracing_subscriber::{util::SubscriberInitExt, EnvFilter};

mod endpoints;
mod globals;
mod indexer;
mod raw;
mod utils;

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

    rweb::serve(endpoints::overview())
        .run(([127, 0, 0, 1], 13000))
        .await;

    Ok(())
}
