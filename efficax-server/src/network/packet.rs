use std::io::{self, Cursor};
use std::{net::SocketAddr};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::OwnedWriteHalf;

use super::data::NetworkData;

use super::data::chat::ChatData;
use super::data::input::InputData;

#[derive(Debug)]
pub struct NetworkPacket {
    pub addr: SocketAddr,
    pub data: NetworkData
}

impl NetworkPacket {
    pub fn new(addr: SocketAddr, data: NetworkData) -> NetworkPacket {
        NetworkPacket {
            addr,
            data
        }
    }

    pub async fn read(addr: SocketAddr, reader: &mut Cursor<&Vec<u8>>) -> io::Result<NetworkPacket> {
        let packet_type = reader.read_u8().await?;
        match packet_type {
            InputData::ID => Ok(NetworkPacket::new(addr, NetworkData::Input(InputData::read(reader).await?))),
            ChatData::ID =>  Ok(NetworkPacket::new(addr, NetworkData::Chat(ChatData::read(reader).await?))),
            _ => Err(io::Error::new(io::ErrorKind::Other, format!("bad packet type: {}", packet_type)))
        }
    }

    pub async fn send(&self, writer: &OwnedWriteHalf) {
        let mut buf = Vec::new();
        match &self.data {
            NetworkData::EntityUpdate(data) => {
                if let Err(_) = data.write(&mut buf).await {
                    println!("[network sender]: error writing data: {:?} to buffer: {:#?} for client: {}", data, buf, self.addr);
                }
                self.send_buf(writer, buf).await;
            }
            data => {
                println!("[network sender]: tried to send unsupported data: {:?} to client: {}", data, self.addr);  
            }
        };
    }

    async fn send_buf(&self, writer: &mut OwnedWriteHalf, buf: Vec<u8>) {
        if let Err(_) = writer.writable().await {
            println!("[network sender]: error waiting for socket to become writable for client: {}", self.addr);
        }
        match writer.write_all(&buf) {
            Ok(_) => (),
            Err(_) => println!("[network sender]: error writing to client: {}", self.addr)
            /*
            Ok(0) => {
                println!("[network sender]: wrote zero bytes to client: {}", self.addr);
            }
            Ok(n) => {
                println!("[network sender]: sent {} bytes to client: {}", n, self.addr);
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                println!("[network sender]: would block while sending to client: {}", self.addr);
            }
            Err(e) => {
                println!("[network sender]: error: {} while writing to client: {}", e, self.addr);
            }
            */
        };
    }
}