use std::{net::SocketAddr};

use super::data::NetworkData;

#[derive(Debug)]
pub struct NetworkPacket {
    pub is_tcp: bool,
    pub addrs: Vec<SocketAddr>,
    pub tick_id: u32,
    pub data: NetworkData
}

impl NetworkPacket {
    pub fn unicast(is_tcp: bool, addr: SocketAddr, tick_id: u32, data: NetworkData) -> Self {
        NetworkPacket {
            is_tcp,
            addrs: vec![addr],
            tick_id,
            data
        }
    }

    pub fn multicast(is_tcp: bool, addrs: Vec<SocketAddr>, tick_id: u32, data: NetworkData) -> Self {
        if addrs.len() == 0 {
            println!("[packet]: provided zero addrs: data: {:?}", data);
        }

        NetworkPacket {
            is_tcp,
            addrs,
            tick_id,
            data
        }
    }

    pub fn get_addr(&self) -> SocketAddr {
        self.addrs[0]
    }
}