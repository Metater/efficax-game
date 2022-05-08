use std::{net::{SocketAddr}, collections::VecDeque, sync::Arc};

use byteorder::{LittleEndian, ByteOrder};
use tokio::{io, task::JoinHandle, sync::{mpsc::UnboundedSender, Notify}, net::{TcpListener, tcp::OwnedReadHalf, UdpSocket}, select};

use crate::network::NetworkClient;

use super::{NetworkReceiverMessage, NetworkSenderMessage, data::NetworkData, packet::NetworkPacket};

const BUF_SIZE: usize = 4096;
const RING_SIZE: usize = 8192;
const UDP_BUF_SIZE: usize = 508;

pub async fn start(receiver_tx: UnboundedSender<NetworkReceiverMessage>, sender_tx: UnboundedSender<NetworkSenderMessage>) -> (Arc<UdpSocket>, Arc<Notify>, JoinHandle<()>, JoinHandle<()>) {
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    let udp_socket = Arc::new(UdpSocket::bind("0.0.0.0:8080").await.unwrap());

    let stop_notifier = Arc::new(Notify::new());

    let udp_receiver_tx = receiver_tx.clone();

    let receiver_stop_notifier = stop_notifier.clone();
    let receiver_handle = tokio::spawn(async move {
        println!("[network receiver]: started");
        start_accepting(listener, receiver_tx, sender_tx, receiver_stop_notifier).await;
        println!("[network receiver]: stopped");
    });

    let receiver_udp_socket = udp_socket.clone();
    let udp_receiver_stop_notifier = stop_notifier.clone();
    let udp_receiver_handle = tokio::spawn(async move {
        println!("[network udp receiver]: started");
        let mut buf = [0u8; UDP_BUF_SIZE];
        loop {
            select! {
                recv_result = receiver_udp_socket.recv_from(&mut buf) => {
                    match recv_result {
                        Ok((len, addr)) => {
                            let packet_slice = &buf[..len];
                            let result: Result<(NetworkData, usize), bincode::error::DecodeError> = bincode::decode_from_slice(packet_slice, bincode::config::legacy());
                            match result {
                                Ok((data, _)) => {
                                    let packet = NetworkPacket::unicast(false, addr, data);
                                    let message = NetworkReceiverMessage::Data(packet);
                                    if udp_receiver_tx.send(message).is_err() {
                                        break;
                                    }
                                }
                                Err(e) => {
                                    // TODO kick client if too many errors?
                                    println!("[network udp receiver]: error: {} client: {}", e, addr);
                                }
                            }
                        }
                        Err(_e) => {
                            continue;
                        }
                    }
                },
                () = udp_receiver_stop_notifier.notified() => {
                    break;
                }
            }
        }
        println!("[network udp receiver]: stopped");
    });

    (udp_socket, stop_notifier, receiver_handle, udp_receiver_handle)
}

async fn start_accepting(listener: TcpListener, receiver_tx: UnboundedSender<NetworkReceiverMessage>, sender_tx: UnboundedSender<NetworkSenderMessage>, stop_notifier: Arc<Notify>) {
    loop {
        select! {
            accept_result = listener.accept() => {
                let (stream, addr) = match accept_result {
                    Ok(client) => client,
                    Err(_) => continue
                };
    
                if let Err(_) = stream.set_nodelay(true) {
                    continue;
                }
        
                let (reader, writer) = stream.into_split();
        
                let receiver_channel = receiver_tx.clone();
                if receiver_channel.send(NetworkReceiverMessage::Join(addr)).is_err() {
                    break;
                }
    
                let mut sender_channel = sender_tx.clone();
                if sender_channel.send(NetworkSenderMessage::Join(NetworkClient::new(addr, writer))).is_err() {
                    break;
                }

                let receiver_stop_notifier = stop_notifier.clone();
                tokio::spawn(async move {
                    if let Err(e) = receive(&receiver_channel, &mut sender_channel, reader, addr, receiver_stop_notifier).await {
                        println!("[network receiver]: error: {} client: {}", e, addr);
                    }
    
                    receiver_channel.send(NetworkReceiverMessage::Leave(addr)).ok();
                    sender_channel.send(NetworkSenderMessage::Leave(addr)).ok();
                });
            },
            () = stop_notifier.notified() => {
                break;
            }
        }
    }
}

async fn receive(receiver_tx: &UnboundedSender<NetworkReceiverMessage>, sender_tx: &mut UnboundedSender<NetworkSenderMessage>, reader: OwnedReadHalf, addr: SocketAddr, stop_notifier: Arc<Notify>) -> io::Result<()> {
    let mut ring_buf: VecDeque<u8> = VecDeque::new();
    loop {
        select! {
            readable_result = reader.readable() => {
                readable_result?;
                let mut buf = [0u8; BUF_SIZE];
                match reader.try_read(&mut buf) {
                    Ok(0) => break,
                    Ok(n) => {
                        //println!("[network receiver]: client: {} sent: {} bytes", addr, n);
                        ring_buf.extend(&buf[..n]);
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                        continue;
                    }
                    Err(e) => {
                        return Err(e.into());
                    }
                }
        
                if ring_buf.capacity() > RING_SIZE {
                    return Err(io::Error::new(io::ErrorKind::Other, "unexpectedly expanded ring buffer"));
                }
        
                let mut available_data = ring_buf.len();
        
                if available_data >= 2 {
                    fn get_packet_size(ring_buf: &VecDeque<u8>) -> u16 {
                        LittleEndian::read_u16(&[*ring_buf.get(0).unwrap() as u8, *ring_buf.get(1).unwrap() as u8])
                    }
        
                    let mut declared_packet_size = get_packet_size(&ring_buf);
                    
                    while available_data >= declared_packet_size as usize + 2 {
                        if declared_packet_size <= 0 {
                            return Err(io::Error::new(io::ErrorKind::Other, "declared packet size too small"));
                        }
                        if declared_packet_size > BUF_SIZE as u16 {
                            return Err(io::Error::new(io::ErrorKind::Other, "declared packet size too large"));
                        }
        
                        let slice = ring_buf.make_contiguous();
                        
                        let packet_slice = &slice[2..(declared_packet_size as usize) + 2];
                        let result: Result<(NetworkData, usize), bincode::error::DecodeError> = bincode::decode_from_slice(packet_slice, bincode::config::legacy());
                        match result {
                            Ok((data, len)) => {
                                match data {
                                    NetworkData::InitUDP(udp_port) => {
                                        if sender_tx.send(NetworkSenderMessage::InitUDP((addr, udp_port))).is_err() {
                                            return Ok(())
                                        }
                                    },
                                    other => {
                                        let packet = NetworkPacket::unicast(true, addr, other);
                                        let message = NetworkReceiverMessage::Data(packet);
                                        if receiver_tx.send(message).is_err() {
                                            return Ok(());
                                        }
                                    }
                                }
                                ring_buf.drain(..len + 2);
                            }
                            Err(e) => {
                                return Err(io::Error::new(io::ErrorKind::Other, format!("unparsable packet: {}", e)));
                            }
                        }
        
                        available_data = ring_buf.len();
                        if available_data >= 2 {
                            declared_packet_size = get_packet_size(&ring_buf);
                        }
                    }
                }
            },
            () = stop_notifier.notified() => {
                break;
            }
        }
    }

    Ok(())
}