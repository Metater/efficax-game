use std::{net::SocketAddr};

use super::data::NetworkData;

pub struct NetworkPacket {
    from: SocketAddr,
    data: Vec<u8>
}

impl NetworkPacket {
    pub fn new(from: SocketAddr, data: Vec<u8>) -> NetworkPacket {
        NetworkPacket {
            from,
            data
        }
    }

    pub fn get(&self) -> Option<NetworkData> {
        let packet_type = self.data.get(0)?;
        match packet_type {
            0 => Some(NetworkData::Test),
            _ => None
        }
    }
}