use std::collections::HashMap;
use std::io;
use std::{net::SocketAddr};
use byteorder::{LittleEndian, ByteOrder};
use tokio::io::{AsyncWriteExt};
use tokio::net::tcp::OwnedWriteHalf;

use super::data::NetworkData;

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

    pub async fn send(&self, clients: &mut HashMap<SocketAddr, OwnedWriteHalf>) {
        let mut buf = [0; 4096];

        let encode_result = bincode::encode_into_slice(&self.data, &mut buf[2..], bincode::config::legacy());

        match encode_result {
            Ok(len) => {
                LittleEndian::write_u16(&mut buf[0..2], len as u16);
                for &addr in &self.addrs {
                    if let Some(writer) = clients.get_mut(&addr) {
                        self.send_to(writer, &buf[..len + 2], addr).await;
                    }
                    else {
                        println!("[network sender]: tried to send data: {:?} to missing client: {}", self.data, addr);
                    }
                }
            }
            Err(e) => {
                println!("[network sender]: error: {} writing data: {:?} for client(s): {:?}", e, self.data, self.addrs);
            }
        }
    }

    async fn send_to(&self, writer: &mut OwnedWriteHalf, buf: &[u8], addr: SocketAddr) {
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
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                println!("[network sender]: would block while sending to client: {}", addr);
            }
            Err(e) => {
                println!("[network sender]: error: {} while writing to client: {}", e, addr);
            }
        };
    }
}