use crate::constants::*;
use crate::epidemic::member::{Labels, Metadata};
use chrono::Duration;
use std::net::{SocketAddr, ToSocketAddrs};

#[derive(Debug, Clone)]
pub struct ClusterConfig {
    pub cluster_key: Vec<u8>,
    pub ping_interval: Duration,
    pub network_mtu: usize,
    pub ping_request_host_count: usize,
    pub ping_timeout: Duration,
    pub listen_addr: SocketAddr,
    /// Labels are key/value pairs that are attached to nodes.
    /// Labels are intended to be used to specify identifying attributes of nodes
    /// that are meaningful and relevant to users.
    pub labels: Labels,
    /// Metadata can be stored a data ( binary, plain text, JSON, etc. ) related to a node as binary array.
    /// The data to be stored in metadata can be freely formatted and used by the user.
    pub metadata: Metadata,
}

impl Default for ClusterConfig {
    fn default() -> Self {
        let directed = SocketAddr::from(([127, 0, 0, 1], CONST_INFECTION_PORT));

        ClusterConfig {
            cluster_key: b"default".to_vec(),
            ping_interval: Duration::seconds(1),
            network_mtu: CONST_PACKET_SIZE,
            ping_request_host_count: 3,
            ping_timeout: Duration::seconds(3),
            listen_addr: directed.to_socket_addrs().unwrap().next().unwrap(),
            labels: Labels::new(),
            metadata: Metadata::new(),
        }
    }
}
