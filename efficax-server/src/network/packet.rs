use std::io::{self, Cursor};
use std::{net::SocketAddr};

use tokio::io::AsyncReadExt;

use super::data::NetworkData;

use super::data::chat::ChatData;
use super::data::input::InputData;

#[derive(Debug)]
pub struct NetworkPacket {
    pub from: SocketAddr,
    pub data: NetworkData
}

impl NetworkPacket {
    pub fn new(from: SocketAddr, data: NetworkData) -> NetworkPacket {
        NetworkPacket {
            from,
            data
        }
    }

    pub async fn parse(from: SocketAddr, reader: &mut Cursor<&Vec<u8>>) -> io::Result<NetworkPacket> {
        let packet_type = reader.read_u8().await?;
        match packet_type {
            InputData::ID => Ok(NetworkPacket::new(from, NetworkData::Input(InputData::parse(reader).await?))),
            ChatData::ID =>  Ok(NetworkPacket::new(from, NetworkData::Chat(ChatData::parse(reader).await?))),
            _ => Err(io::Error::new(io::ErrorKind::Other, format!("bad packet type: {}", packet_type)))
        }
    }
}