pub mod handler;
pub mod util;

use {
    anyhow::Result,
    btclib::types::Blockchain,
    clap::Parser,
    dashmap::DashMap,
    static_init::dynamic,
    std::path::Path,
    tokio::{
        net::{TcpListener, TcpStream},
        sync::RwLock,
    },
};

#[dynamic]
pub static BLOCKCHAIN: RwLock<Blockchain> = RwLock::new(Blockchain::new());
// Node pool
#[dynamic]
pub static NODES: DashMap<String, TcpStream> = DashMap::new();

#[derive(Parser)]
/// A toy blockchain node
struct Cli {
    #[clap(long, default_value_t = 9000)]
    /// port number
    pub port: u16,
    #[clap(long, default_value = "./data/blockchain.cbor")]
    /// blockchain file location
    pub blockchain_file: String,
    #[clap(required = false)]
    /// addresses of initial nodes
    pub nodes: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let args: Cli = Cli::parse();
    // Access the parsed arguments
    let port = args.port;
    let blockchain_file = args.blockchain_file;
    let nodes = args.nodes;

    // Check if the blockchain_file exists
    if Path::new(&blockchain_file).exists() {
        util::load_blockchain(&blockchain_file).await?;
    } else {
        println!("blockchain file does not exist!");
        util::populate_connections(&nodes).await?;
        println!("total amount of known nodes: {}", NODES.len());

        if nodes.is_empty() {
            println!("no initial nodes provided, starting as a seed node");
        } else {
            let (longest_name, longest_count) = util::find_longest_chain_node().await?;
            // request the blockchain from the node with the longest blockchain
            util::download_blockchain(&longest_name, longest_count).await?;
            println!("blockchain downloaded from {}", longest_name);
            // recalculate utxos
            {
                let mut blockchain = BLOCKCHAIN.write().await;
                blockchain.rebuild_utxos();
            }
            // try to adjust difficulty
            {
                let mut blockchain = BLOCKCHAIN.write().await;
                blockchain.try_adjust_target();
            }
        }
    }

    // start a task to periodically cleanup the mempool
    // normally, you would want to keep and join the handle
    tokio::spawn(util::cleanup());
    // and a task to periodically save the blockchain
    tokio::spawn(util::save(blockchain_file.clone()));

    // Start the TCP listener on 0.0.0.0:port
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await?;
    println!("Listening on {}", addr);

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(handler::handle_connection(socket));
    }
}
