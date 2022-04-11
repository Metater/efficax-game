use std::{net::SocketAddr};

use tokio::net::tcp::OwnedWriteHalf;

use crate::network::packet::NetworkPacket;

pub struct NetworkClient {
    addr: SocketAddr,
    writer: OwnedWriteHalf
}

impl NetworkClient {
    pub fn new(addr: SocketAddr, writer: OwnedWriteHalf) -> NetworkClient {
        NetworkClient {
            addr,
            writer
        }
    }
}
