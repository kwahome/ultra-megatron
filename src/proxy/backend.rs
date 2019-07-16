use std::net::SocketAddr;
use std::collections::HashMap;
use std::io::{Error, ErrorKind};

#[derive(Debug, Clone)]
struct Wrr {
    healthy: bool,
    weight: u16,
    weights_index: usize,
}

#[derive(Debug, Clone)]
pub struct ServerPool {
    servers_map: HashMap<SocketAddr, Wrr>,
    weights: Vec<u16>,
    weighted_servers: Vec<SocketAddr>,
}

#[derive(Debug)]
pub struct Backend {
    pub name: String,
    pub servers: Arc<RwLock<ServerPool>>,
    pub health_check_interval: u64,
}
