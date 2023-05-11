use std::{
    sync::Arc,
    time::Duration,
    net::{self, SocketAddr, IpAddr, Ipv4Addr},
    collections::HashMap,
    fs,
};

use parking_lot::RwLock;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use tokio::{
    net::TcpListener,
    sync::{mpsc, oneshot},
    time::timeout,
    io::{AsyncReadExt, AsyncWriteExt},
};

use serde_json;

use crate::{Account, DBHandler, DBPool, Pubkey, ShardPath, RateLimiter, EncodedPubkey};

const TIMEOUT_SEC: u64 = 5;

#[derive(Debug)]
struct Cluster {
    name: String,
    nodes: HashMap<EncodedPubkey, Arc<RwLock<Node>>>,
}

impl Serialize for Cluster {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        #[derive(Debug, Serialize)]
        struct Tmp<'a> {
            name: &'a str,
            nodes: HashMap<EncodedPubkey, Node>,
        }

        let mut hashmap: HashMap<EncodedPubkey, Node> = HashMap::new();
        for (k, node) in &self.nodes {
            let node_ref = &*node.read();
            hashmap.insert(k.clone(), node_ref.clone());
        }

        let new = Tmp {
            name: &self.name,
            nodes: hashmap,
        };
        new.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Cluster {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        struct Tmp {
            name: String,
            nodes: HashMap<EncodedPubkey, Node>,
        }

        let tmp: Tmp = serde::Deserialize::deserialize(deserializer)?;
        let mut nodes: HashMap<EncodedPubkey, Arc<RwLock<Node>>> = HashMap::new();
        for (k, v) in tmp.nodes {
            nodes.insert(k, Arc::new(RwLock::new(v)));
        }

        Ok(Cluster {
            name: tmp.name,
            nodes,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub node_id: EncodedPubkey,
    pub address: SocketAddr,
    pub key: String,
    pub shard: String,
    pub leader: bool,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            node_id: EncodedPubkey::from(Pubkey::new_rand()),
            address: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0),
            key: String::new(),
            shard: String::new(),
            leader: false,
        }
    }
}

impl Node {
    pub fn new(address: SocketAddr, key: String, shard: String, leader: bool) -> Self {
        let node_id = EncodedPubkey::from(Pubkey::new_rand());

        Self {
            node_id,
            address,
            key,
            shard,
            leader,
        }
    }
}

#[derive(Clone)]
struct Peer {
    node: Arc<RwLock<Node>>,
}

impl Peer {
    pub fn new(stream: tokio::net::TcpStream, node: Arc<RwLock<Node>>) -> Self {
        // initialize new peer from incoming stream
        let cloned_node = Arc::clone(&node);
        tokio::spawn(async move {
            // read data from stream and process it
            let peer = Peer { node: cloned_node };
            peer.handle_data(stream).await;
        });

        Peer { node }
    }

    async fn handle_data(&self, mut stream: tokio::net::TcpStream) {
        loop {
            let mut buf = [0u8; 1024];
            let n = match stream.read(&mut buf).await {
                Ok(n) if n == 0 => return,
                Ok(n) => n,
                Err(e) => {
                    eprintln!("failed to read from socket; err = {:?}", e);
                    return;
                }
            };

            let request = String::from_utf8_lossy(&buf[..n]);
            if request.starts_with("GET_NODE") {
                // Handle GET_NODE request
                let response = self.get_node().await;
                if let Err(e) = stream.write_all(response.as_bytes()).await {
                    eprintln!("failed to write response to socket; err = {:?}", e);
                    return;
                }
            } else if request.starts_with("SET_NODE") {
                // Handle SET_NODE request
                // ...
            } else {
                // Unknown request
                // ...
            }
        }
    }

    pub async fn connect(addr: SocketAddr, node: Arc<RwLock<Node>>) -> Result<Self, Box<dyn std::error::Error>> {
        let stream = tokio::net::TcpStream::connect(addr).await?;
        let peer = Peer::new(stream, node);
        Ok(peer)
    }

    async fn get_node(&self) -> String {
        let node = self.node.read();
        let node_str = serde_json::to_string(&*node).unwrap();
        format!("NODE {}\n", node_str)
    }


    // pub async fn bootstrap(&self) -> Result<(String, Vec<SocketAddr>), Box<dyn std::error::Error>> {
    //     // send bootstrap request to peer and wait for response
    //     let shard_path = String::from("some shard path");
    //     let peer_list = vec![SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 12345)];
    //
    //     Ok((shard_path, peer_list))
    // }
}

pub async fn bootstrap(port: u16, boot_addr: SocketAddr, cluster_json: &str) -> Result<(), Box<dyn std::error::Error>> {
    let (shutdown_tx, shutdown_rx) = oneshot::channel();

    // let node_addr: SocketAddr = format!("0.0.0.0:{}", port).parse()?;
    // let mut listener = TcpListener::bind(&node_addr).await?;
    let (new_peer_tx, mut new_peer_rx) = mpsc::unbounded_channel();

    // Load the cluster info from JSON
    let cluster: Cluster = serde_json::from_str(cluster_json)?;

    // // spawn task to accept incoming connections
    // let new_peer_tx_clone = new_peer_tx.clone();
    // let handler = tokio::spawn(async move {
    //     while let Ok((stream, _addr)) = listener.accept().await {
    //         let new_peer_tx = new_peer_tx_clone.clone();
    //         let node = Arc::new(RwLock::new(Node::default()));
    //         let peer = Peer::new(stream, node);
    //         if new_peer_tx.send(Arc::new(peer)).is_err() {
    //             return;
    //         }
    //     }
    // });

    for node_id in cluster.nodes.keys() {
        // Find the corresponding node info
        let node = Arc::clone(cluster.nodes.get(&node_id).unwrap());

        let new_peer_tx = new_peer_tx.clone();
        let boot_addr = node.read().address;
        tokio::spawn(async move {
            match Peer::connect(boot_addr, node).await {
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
            // Process the new peer by sending appropriate requests based on cluster information
            let peer_node = Arc::clone(&peer.node);

            // Send requests to the new peer based on node information
            // ...

            // Example: Sending a GET_NODE request
            let get_node_request = "GET_NODE".to_string();
            let mut buf = get_node_request.into_bytes();
            buf.resize(1024, 0); // Resize the buffer to a fixed size
            // if let Err(e) = peer.write_all(&buf).await {
            //     eprintln!("failed to send GET_NODE request to peer; err = {:?}", e);
            // }
        }
    });

    // wait for shutdown signal
    shutdown_rx.await?;

    // shutdown handler tasks
    handler.abort();

    Ok(())
}