use std::{net::SocketAddr};

use tokio::net::tcp::OwnedWriteHalf;

pub struct NetworkClient {
    pub addr: SocketAddr,
    pub writer: OwnedWriteHalf
}

impl NetworkClient {
    pub fn new(addr: SocketAddr, writer: OwnedWriteHalf) -> NetworkClient {
        NetworkClient {
            addr,
            writer
        }
    }
}
