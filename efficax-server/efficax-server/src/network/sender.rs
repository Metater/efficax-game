use std::collections::HashMap;
use std::io;
use std::net::SocketAddr;
use std::sync::Arc;

use byteorder::{LittleEndian, ByteOrder};
use tokio::net::UdpSocket;
use tokio::{task::JoinHandle, sync::mpsc::UnboundedReceiver};

use super::{NetworkSenderMessage, NetworkClient};
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
    let mut clients: HashMap<SocketAddr, NetworkClient> = HashMap::new();
    while let Some(message) = sender_rx.recv().await {
        match message {
            NetworkSenderMessage::Join(client) => {
                clients.insert(client.addr, client);
            }
            NetworkSenderMessage::Leave(addr) => {
                clients.remove(&addr);
            }
            NetworkSenderMessage::InitUDP((addr, udp_port)) => {
                if let Some(client) = clients.get_mut(&addr) {
                    if client.udp_port == 0 {
                        client.udp_port = udp_port;
                    }
                }
            }
            NetworkSenderMessage::Data(packet) => {
                let mut buf = [0; 4096];
                let len = encode_packet(&packet, &mut buf);
                let encoded_data = &buf[..len];

                for addr in &packet.addrs {
                    if let Some(client) = clients.get_mut(addr) {
                        if packet.is_tcp {
                            tcp_send(&packet, encoded_data, client).await;
                        }
                        else if client.udp_port == 0 {
                            // Drop data, UDP is unreliable, so it won't matter
                        }
                        else {
                            udp_send(&packet, &encoded_data[2..], client, &udp_socket).await;
                        }
                    }
                    else {
                        println!("[network sender]: tried to send data: {:?} to missing client: {}", packet.data, addr);
                    }
                }
            }
            NetworkSenderMessage::Stop => {
                break;
            }
        };
    }
}

fn encode_packet(packet: &NetworkPacket, buf: &mut [u8]) -> usize {
    let encode_result = bincode::encode_into_slice(&packet.data, &mut buf[3..], bincode::config::legacy());

    match encode_result {
        Ok(len) => {
            LittleEndian::write_u16(&mut buf[..2], len as u16);
            buf[2] = packet.tick_id;
            len + 3
        }
        Err(e) => {
            panic!("[network sender]: error: {} encoding data: {:?} for client(s): {:?}", e, packet.data, packet.addrs);
        }
    }
}

async fn tcp_send(packet: &NetworkPacket, encoded_data: &[u8], client: &mut NetworkClient) {
    if let Err(_) = client.writer.writable().await {
        println!("[network sender]: error waiting for socket to become writable for client: {}", client.addr);
    }
    match client.writer.try_write(encoded_data) {
        Ok(0) => {
            println!("[network sender]: wrote zero bytes to client: {}", client.addr);
        }
        Ok(_n) => {
            //println!("[network sender]: sent {} bytes to client: {}", n, client.addr);
        }
        Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
            println!("[network sender]: would block while sending to client: {}", client.addr);
        }
        Err(e) => {
            println!("[network sender]: error: {} while writing data: {:?} to client: {}", e, packet.data, client.addr);
        }
    }
}

async fn udp_send(packet: &NetworkPacket, encoded_data: &[u8], client: &mut NetworkClient, socket: &Arc<UdpSocket>) {
    let udp_addr = SocketAddr::new(client.addr.ip(), client.udp_port);
    if let Err(e) = socket.send_to(encoded_data, udp_addr).await {
        println!("[network sender]: error: {} while writing udp data: {:?} to client: {}", e, packet.data, client.addr);
    }
}