use std::collections::HashMap;
use std::io;
use std::net::SocketAddr;
use std::sync::Arc;

use byteorder::{LittleEndian, ByteOrder};
use tokio::net::UdpSocket;
use tokio::net::tcp::OwnedWriteHalf;
use tokio::{task::JoinHandle, sync::mpsc::UnboundedReceiver};

use super::NetworkSenderMessage;
use super::packet::NetworkPacket;

pub async fn start(sender_rx: UnboundedReceiver<NetworkSenderMessage>, udp_socket: Arc<UdpSocket>) -> JoinHandle<()> {
    let handle = tokio::spawn(async move {
        println!("[network sender]: started");
        start_sending(sender_rx, udp_socket).await;
        println!("[network sender]: stopped");
    });

    handle
}

async fn start_sending(mut sender_rx: UnboundedReceiver<NetworkSenderMessage>, udp_socket: Arc<UdpSocket>) {
    let mut clients: HashMap<SocketAddr, OwnedWriteHalf> = HashMap::new();
    while let Some(message) = sender_rx.recv().await {
        match message {
            NetworkSenderMessage::Join((addr, writer)) => {
                clients.insert(addr, writer);
            }
            NetworkSenderMessage::Leave(addr) => {
                clients.remove(&addr);
            }
            NetworkSenderMessage::Data(packet) => {
                let (buf, len) = encode_packet(&packet);
                let encoded_data = &buf[..len];
                if packet.is_tcp {
                    tcp_send(&packet, encoded_data, &mut clients).await;
                }
                else {
                    println!("sent udp data: {:?}", encoded_data);
                    udp_send(&packet, encoded_data, &udp_socket).await;
                }
            }
            NetworkSenderMessage::Stop => {
                break;
            }
        };
    }
}

fn encode_packet(packet: &NetworkPacket) -> ([u8; 4096], usize) {
    let mut buf = [0; 4096];

    let encode_result = bincode::encode_into_slice(&packet.data, &mut buf[2..], bincode::config::legacy());

    match encode_result {
        Ok(len) => {
            LittleEndian::write_u16(&mut buf[..2], len as u16);
            (buf, len + 2)
        }
        Err(e) => {
            panic!("[network sender]: error: {} encoding data: {:?} for client(s): {:?}", e, packet.data, packet.addrs);
        }
    }
}

async fn tcp_send(packet: &NetworkPacket, encoded_data: &[u8], clients: &mut HashMap<SocketAddr, OwnedWriteHalf>) {
    for &addr in &packet.addrs {
        if let Some(writer) = clients.get_mut(&addr) {
            if let Err(_) = writer.writable().await {
                println!("[network sender]: error waiting for socket to become writable for client: {}", addr);
            }
            match writer.try_write(encoded_data) {
                Ok(0) => {
                    println!("[network sender]: wrote zero bytes to client: {}", addr);
                }
                Ok(_n) => {
                    //println!("[network sender]: sent {} bytes to client: {}", n, addr);
                }
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                    println!("[network sender]: would block while sending to client: {}", addr);
                }
                Err(e) => {
                    println!("[network sender]: error: {} while writing data: {:?} to client: {}", e, packet.data, addr);
                }
            }
        }
        else {
            println!("[network sender]: tried to send data: {:?} to missing client: {}", packet.data, addr);
        }
    }
}

async fn udp_send(packet: &NetworkPacket, encoded_data: &[u8], socket: &Arc<UdpSocket>) {
    for &addr in &packet.addrs {
        println!("try send to: {} ", addr);
        if let Err(e) = socket.send_to(encoded_data, addr).await {
            println!("[network sender]: error: {} while writing udp data: {:?} to client: {}", e, packet.data, addr);
        }
    }
}