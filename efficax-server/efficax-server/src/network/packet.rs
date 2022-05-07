use std::{net::SocketAddr};

use super::data::NetworkData;

#[derive(Debug)]
pub struct NetworkPacket {
    pub is_tcp: bool,
    pub addrs: Vec<SocketAddr>,
    pub data: NetworkData
}

impl NetworkPacket {
    pub fn unicast(is_tcp: bool, addr: SocketAddr, data: NetworkData) -> Self {
        NetworkPacket {
            is_tcp,
            addrs: vec![addr],
            data
        }
    }

    pub fn multicast(is_tcp: bool, addrs: Vec<SocketAddr>, data: NetworkData) -> Self {
        NetworkPacket {
            is_tcp,
            addrs,
            data
        }
    }

    pub fn get_addr(&self) -> SocketAddr {
        self.addrs[0]
    }
}