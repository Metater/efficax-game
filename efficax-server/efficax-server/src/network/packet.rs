use std::collections::HashMap;
use std::io::{self, Cursor};
use std::{net::SocketAddr};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::OwnedWriteHalf;

use super::data::NetworkData;

use super::data::chat::ChatData;
use super::data::input::InputData;

#[derive(Debug)]
pub struct NetworkPacket {
    pub addrs: Vec<SocketAddr>,
    pub data: NetworkData
}

impl NetworkPacket {
    pub fn unicast(addr: SocketAddr, data: NetworkData) -> Self {
        NetworkPacket {
            addrs: vec![addr],
            data
        }
    }

    pub fn multicast(addrs: Vec<SocketAddr>, data: NetworkData) -> Self {
        NetworkPacket {
            addrs,
            data
        }
    }

    pub async fn read(addr: SocketAddr, reader: &mut Cursor<&Vec<u8>>) -> io::Result<NetworkPacket> {
        let packet_type = reader.read_u8().await?;
        match packet_type {
            InputData::ID => Ok(NetworkPacket::unicast(addr, NetworkData::Input(InputData::read(reader).await?))),
            ChatData::ID =>  Ok(NetworkPacket::unicast(addr, NetworkData::Chat(ChatData::read(reader).await?))),
            _ => Err(io::Error::new(io::ErrorKind::Other, format!("bad packet type: {}", packet_type)))
        }
    }

    pub async fn send(&self, clients: &mut HashMap<SocketAddr, OwnedWriteHalf>) {
        let mut buf = Vec::new();
        match &self.data {
            // this block also contains general stuff?
            NetworkData::TickUpdate(data) => {
                if let Err(_) = data.write(&mut buf).await {
                    println!("[network sender]: error writing data: {:?} to buffer: {:#?} for client(s): {:?}", data, buf, self.addrs);
                }
                for &addr in &self.addrs {
                    if let Some(writer) = clients.get_mut(&addr) {
                        self.send_buf(writer, &buf, addr).await;
                    }
                    else {
                        println!("[network sender]: tried to send data: {:?} to missing client: {}", self.data, addr);
                    }
                }
            }
            data => {
                println!("[network sender]: tried to send unsupported data: {:?} to client(s): {:?}", data, self.addrs);  
            }
        };
    }

    async fn send_buf(&self, writer: &mut OwnedWriteHalf, buf: &Vec<u8>, addr: SocketAddr) {
        if let Err(_) = writer.writable().await {
            println!("[network sender]: error waiting for socket to become writable for client: {}", addr);
        }
        match writer.write(buf).await {
            Ok(0) => {
                println!("[network sender]: wrote zero bytes to client: {}", addr);
            }
            Ok(n) => {
                println!("[network sender]: sent {} bytes to client: {}", n, addr);
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                println!("[network sender]: would block while sending to client: {}", addr);
            }
            Err(e) => {
                println!("[network sender]: error: {} while writing to client: {}", e, addr);
            }
        };
    }
}