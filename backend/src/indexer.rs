use std::{
    collections::HashMap,
    path::Path,
    str::FromStr,
    sync::Arc,
    time::{Duration, Instant},
};

use rusqlite::{params, OptionalExtension};
use smol::lock::Mutex;
use stdcode::StdcodeSerializeExt;
use themelio_nodeprot::ValClient;
use themelio_structs::{CoinID, CoinValue, Denom};

/// The indexer encapsulates an SQLite database and is able to query info that thin clients cannot effectively calculate.
pub struct Indexer {
    db: Arc<Mutex<rusqlite::Connection>>,
    _task: smol::Task<()>,
}

impl Indexer {
    /// Creates a new indexer at a given path
    pub fn new(path: &Path, client: ValClient) -> Self {
        let conn = rusqlite::Connection::open(path).unwrap();
        conn.query_row("pragma journal_mode=WAL", [], |_| Ok(()))
            .unwrap();
        conn.execute("pragma synchronous=NORMAL", []).unwrap();
        let db = Arc::new(Mutex::new(conn));
        let db2 = db.clone();
        let _task = smolscale::spawn(async move { indexer_task(db2, client).await.unwrap() });
        Self { db, _task }
    }
}

async fn indexer_task(
    db: Arc<Mutex<rusqlite::Connection>>,
    client: ValClient,
) -> anyhow::Result<()> {
    // Create the tables
    {
        let conn = db.lock().await;
        conn.execute("create table if not exists coins (txhash not null, idx not null, denom not null, value not null, additional_data not null, create_height not null, spent_by)", params![])?;
        conn.execute("create table if not exists token_supplies (height not null, denom not null, sum not null)", params![])?;
        conn.execute(
            "create index if not exists supply_index on token_supplies (height, denom)",
            params![],
        )?;
        conn.execute(
            "create table if not exists headers (height primary key not null, raw not null)",
            params![],
        )?;
    }
    // index blocks in a loop
    loop {
        if let Err(err) = indexer_once(db.clone(), client.clone()).await {
            log::error!("{:?}", err);
        }
    }
}

async fn indexer_once(
    db: Arc<Mutex<rusqlite::Connection>>,
    client: ValClient,
) -> anyhow::Result<()> {
    let snap = client.snapshot().await?;
    {
        let conn = db.lock().await;
        if conn
            .query_row(
                "select raw from headers where height = $1",
                params![snap.current_header().height.0],
                |_| Ok(()),
            )
            .optional()?
            .is_some()
        {
            // pause and die
            smol::Timer::after(Duration::from_secs(1)).await;
            return Ok(());
        }
    }
    // okay, so now there's most likely something to query
    let next_height = {
        let conn = db.lock().await;
        let max_height: u64 = conn
            .query_row(
                "select coalesce(max(height), 0) from headers",
                params![],
                |r| r.get(0),
            )
            .optional()?
            .unwrap_or_default();
        max_height
    } + 1;
    let start = Instant::now();
    let full_block = snap
        .get_older(next_height.into())
        .await?
        .current_block()
        .await?;
    log::debug!(
        "indexing block {} gotten in {:?}",
        next_height,
        start.elapsed()
    );
    {
        let reward_coin = snap
            .get_older(next_height.into())
            .await?
            .get_coin(CoinID::proposer_reward(next_height.into()))
            .await?;
        let reward_amount = reward_coin.map(|v| v.coin_data.value).unwrap_or_default();
        let mut conn = db.lock().await;
        let mut supply: HashMap<Denom, f64> = HashMap::new();
        let txn = conn.transaction()?;
        // Set the header
        txn.execute(
            "insert into headers values ($1, $2)",
            params![next_height, full_block.header.stdcode()],
        )?;

        defmac::defmac!(token_supply denom => {
            supply.entry(denom).or_insert_with(|| {
                txn.query_row(
                    "select sum from token_supplies where denom = $1 and height <= $2 order by height desc limit 1",
                    params![denom.to_string(), next_height.saturating_sub(1)],
                    |r| r.get(0),
                )
                .optional()
                .unwrap()
                .unwrap_or_default()
            })
        });

        *token_supply!(Denom::Mel) += reward_amount.0 as f64 / 1_000_000.0;

        // process ALL the transactions
        for tx in full_block.transactions {
            // process coins
            let txhash = tx.hash_nosigs();
            for input in tx.inputs {
                let _ = txn.execute(
                    "update coins set spent_by = $1 where txhash = $2 and idx = $3",
                    params![txhash.to_string(), input.txhash.to_string(), input.index],
                );
                // update balance
                if let Some((denom, value)) = txn
                    .query_row(
                        "select denom, value from coins where txhash = $1 and idx = $2",
                        params![input.txhash.to_string(), input.index],
                        |r| Ok((r.get(0)?, r.get(1)?)),
                    )
                    .optional()?
                {
                    let denom: String = denom;
                    let value: String = value;
                    let value = CoinValue(value.parse()?);
                    let denom = Denom::from_str(&denom)?;
                    let balance = token_supply!(denom);
                    *balance -= value.0 as f64 / 1_000_000.0;
                }
            }
            for (i, output) in tx.outputs.into_iter().enumerate() {
                txn.execute(
                    "insert into coins values ($1, $2, $3, $4, $5, $6, null)",
                    params![
                        txhash.to_string(),
                        i,
                        output.denom.to_string(),
                        output.value.0.to_string(),
                        output.additional_data,
                        next_height,
                    ],
                )?;
                let balance = token_supply!(output.denom);
                *balance += output.value.0 as f64 / 1_000_000.0;
            }
        }
        for (coin, supply) in supply {
            txn.execute(
                "insert into token_supplies values ($1, $2, $3)",
                params![next_height, coin.to_string(), supply],
            )?;
        }
        txn.commit()?;
    }
    Ok(())
}
