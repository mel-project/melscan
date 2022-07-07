use std::net::SocketAddr;

use once_cell::sync::Lazy;
use structopt::StructOpt;
use themelio_nodeprot::ValClient;
use themelio_structs::NetID;

#[derive(StructOpt)]
pub struct Args {
    #[structopt(long)]
    /// Where to listen for incoming REST API calls
    pub listen: SocketAddr,

    #[structopt(long)]
    /// A full node to connect to
    connect: SocketAddr,

    #[structopt(long)]
    /// Whether or not the block explorer is connected to a testnet node.
    testnet: bool,
}

/// Command-line arguments that were initially passed in.
pub static CMD_ARGS: Lazy<Args> = Lazy::new(Args::from_args);

/// The global ValClient for talking to the network.
pub static CLIENT: Lazy<ValClient> = Lazy::new(|| {
    let client = ValClient::new(
        if CMD_ARGS.testnet {
            NetID::Testnet
        } else {
            NetID::Mainnet
        },
        CMD_ARGS.connect,
    );
    if CMD_ARGS.testnet {
        client.trust(themelio_bootstrap::checkpoint_height(NetID::Testnet).unwrap());
    } else {
        client.trust(themelio_bootstrap::checkpoint_height(NetID::Mainnet).unwrap());
    }
    client
});
