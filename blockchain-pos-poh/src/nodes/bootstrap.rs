use {
    std::{
        sync::Arc,
        time::Duration,
        net::SocketAddr,
    },
    crate::{Account, DBHandler, DBPool, Pubkey, ShardPath, RateLimiter},
};

use tokio::{
    net::TcpListener,
    sync::{mpsc, oneshot},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args();
    args.next(); // skip program name

    // parse arguments
    let port: u16 = args.next().expect("port must be specified").parse()?;
    let node_addr: SocketAddr = format!("0.0.0.0:{}", port).parse()?;
    let boot_addr: SocketAddr = args
        .next()
        .expect("bootstrap node must be specified")
        .parse()?;

    let (shutdown_tx, shutdown_rx) = oneshot::channel();

    let mut listener = TcpListener::bind(&node_addr).await?;
    let (new_peer_tx, mut new_peer_rx) = mpsc::unbounded_channel();

    // spawn task to accept incoming connections
    let new_peer_tx_clone = new_peer_tx.clone();
    let handler = tokio::spawn(async move {
        while let Ok((stream, _addr)) = listener.accept().await {
            let new_peer_tx = new_peer_tx_clone.clone();
            tokio::spawn(async move {
                let peer = Peer::new(stream);
                if new_peer_tx.send(Arc::new(peer)).is_err() {
                    // channel was closed, shutdown
                    return;
                }
            });
        }
    });

    // connect to bootstrap node
    let peer = Peer::connect(boot_addr).await?;

    // send bootstrap request and wait for response
    let (shard_path, peer_list) = peer.bootstrap().await?;

    // add returned peers to new_peer_tx
    for peer_addr in peer_list {
        let new_peer_tx = new_peer_tx.clone();
        let boot_addr = peer_addr.clone();
        tokio::spawn(async move {
            match Peer::connect(boot_addr).await {
                Ok(peer) => {
                    if new_peer_tx.send(Arc::new(peer)).is_err() {
                        // channel was closed, shutdown
                        return;
                    }
                }
                Err(_err) => {
                    // failed to connect, do nothing
                }
            }
        });
    }

    // spawn task to handle new peers
    let handler = tokio::spawn(async move {
        while let Some(peer) = new_peer_rx.recv().await {
            // do something with new peer
        }
    });

    // wait for shutdown signal
    shutdown_rx.await?;

    // shutdown handler tasks
    handler.abort();

    Ok(())
}

struct Peer {}

impl Peer {
    fn new(stream: tokio::net::TcpStream) -> Self {
        // initialize new peer from incoming stream
        Peer {}
    }

    async fn connect(addr: std::net::SocketAddr) -> Result<Self, Box<dyn std::error::Error>> {
        // connect to remote peer at address
        Ok(Peer {})
    }

    async fn bootstrap(&self) -> Result<(String, Vec<SocketAddr>), Box<dyn std::error::Error>> {
        // send bootstrap request to peer and wait for response
        let shard_path = String::from("some shard path");
        let peer_list = vec![SocketAddr::new(std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)), 12345)];
        Ok((shard_path, peer_list))
    }
}