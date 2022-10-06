use std::{net::SocketAddr, path::PathBuf};

use melblkidx::Indexer;
use melnet2::{wire::tcp::TcpBackhaul, Backhaul};
use once_cell::sync::Lazy;
use structopt::StructOpt;
use themelio_nodeprot::{NodeRpcClient, ValClient};
use themelio_structs::NetID;

use crate::backend::Backend;

#[derive(StructOpt)]
pub struct Args {
    #[structopt(long)]
    /// Where to listen for incoming REST API calls
    pub listen: SocketAddr,

    #[structopt(long)]
    /// A full node to connect to
    connect: SocketAddr,

    #[structopt(long)]
    /// The custom network to connect to
    network: Option<NetID>,

    #[structopt(long)]
    /// Whether or not the block explorer is connected to a testnet node.
    testnet: bool,

    #[structopt(long)]
    /// If set, indexes blocks and saves them to the given location.
    blkidx_db: Option<PathBuf>,
}

/// Command-line arguments that were initially passed in.
pub static CMD_ARGS: Lazy<Args> = Lazy::new(Args::from_args);

/// The global ValClient for talking to the network.
pub static CLIENT: Lazy<ValClient> = Lazy::new(|| {
    smolscale::block_on(async move {
        let backhaul = TcpBackhaul::new();
        let network = if let Some(custom_net) = CMD_ARGS.network {
            custom_net
        } else if CMD_ARGS.testnet {
            NetID::Testnet
        } else {
            NetID::Mainnet
        };

        let client = ValClient::new(
            network,
            NodeRpcClient(
                backhaul
                    .connect(CMD_ARGS.connect.to_string().into())
                    .await
                    .unwrap(),
            ),
        );
        if let Some(_) = CMD_ARGS.network {
            println!("Insecurely trusting snapshot on a custom network");
            client.insecure_latest_snapshot().await.unwrap();
        } else if CMD_ARGS.testnet {
            client.trust(themelio_bootstrap::checkpoint_height(NetID::Testnet).unwrap());
        } else {
            client.trust(themelio_bootstrap::checkpoint_height(NetID::Mainnet).unwrap());
        }
        client
    })
});

pub static BACKEND: Lazy<Backend> = Lazy::new(|| {
    Backend::new(
        CLIENT.clone(),
        CMD_ARGS
            .blkidx_db
            .as_ref()
            .map(|path| Indexer::new(path, CLIENT.clone()).unwrap()),
    )
});
