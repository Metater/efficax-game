use std::{net::{SocketAddr}, collections::VecDeque, sync::Arc};

use byteorder::{LittleEndian, ByteOrder};
use tokio::{io, task::JoinHandle, sync::{mpsc::UnboundedSender, Notify}, net::{TcpListener, tcp::OwnedReadHalf}, select};

use crate::network::data::InputData;

use super::{NetworkReceiverMessage, NetworkSenderMessage, data::NetworkData, packet::NetworkPacket};

const BUF_SIZE: usize = 4096;
const RING_SIZE: usize = 8192;

pub async fn start(listener_tx: UnboundedSender<NetworkReceiverMessage>, sender_tx: UnboundedSender<NetworkSenderMessage>) -> (Arc<Notify>, JoinHandle<()>) {
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

    let stop_notifier = Arc::new(Notify::new());
    let accepter_stop_notifier = stop_notifier.clone();

    let handle = tokio::spawn(async move {
        start_accepting(listener, listener_tx, sender_tx, accepter_stop_notifier).await;
        println!("[network listener]: stopped");
    });

    (stop_notifier, handle)
}

async fn start_accepting(listener: TcpListener, listener_tx: UnboundedSender<NetworkReceiverMessage>, sender_tx: UnboundedSender<NetworkSenderMessage>, stop_notifier: Arc<Notify>) {
    println!("[network listener]: started");
    let mut receive_tasks = Vec::new();
    loop {
        select! {
            () = stop_notifier.notified() => {
                break;
            },
            accept_result = listener.accept() => {
                let (stream, addr) = match accept_result {
                    Ok(client) => client,
                    Err(_) => continue
                };
    
                if let Err(_) = stream.set_nodelay(true) {
                    continue;
                }
                
                ///////////////////////////////////////// UDP THING HERE ////////////////////////////////////////
        
                let (reader, writer) = stream.into_split();
        
                let listener_channel = listener_tx.clone();
                if let Err(_) = listener_channel.send(NetworkReceiverMessage::Join(addr)) {
                    break;
                }
    
                let sender_channel = sender_tx.clone();
                if let Err(_) = sender_channel.send(NetworkSenderMessage::Join((addr, writer))) {
                    break;
                }

                receive_tasks.push(tokio::spawn(async move {
                    if let Err(e) = receive(&listener_channel, reader, addr).await {
                        println!("[network listener]: error: {} client: {}", e, addr);
                    }
    
                    listener_channel.send(NetworkReceiverMessage::Leave(addr)).ok();
                    sender_channel.send(NetworkSenderMessage::Leave(addr)).ok();
                }));
            }
        }
    }
    for receive_task in receive_tasks {
        receive_task.abort();
    }
}

async fn receive(receiver_tx: &UnboundedSender<NetworkReceiverMessage>, reader: OwnedReadHalf, addr: SocketAddr) -> io::Result<()> {
    let mut ring_buf: VecDeque<u8> = VecDeque::new();
    loop {
        reader.readable().await?;
        let mut buf = [0u8; BUF_SIZE];
        match reader.try_read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                println!("[network listener]: client: {} sent: {} bytes", addr, n);
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

        let available_data = ring_buf.len();

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
                
                let packet_slice = &slice[2..2 + declared_packet_size as usize];
                
                let result: Result<(NetworkData, usize), bincode::error::DecodeError> = bincode::decode_from_slice(packet_slice, bincode::config::legacy());
                match result {
                    Ok((data, len)) => {
                        let packet = NetworkPacket::unicast(addr, data);
                        let message = NetworkReceiverMessage::Data(packet);
                        if let Err(_) = receiver_tx.send(message) {
                            return Ok(());
                        }
                        ring_buf.drain(..len + 2);
                    }
                    Err(e) => {
                        return Err(io::Error::new(io::ErrorKind::Other, format!("unparsable packet: {}", e)));
                    }
                }
                declared_packet_size = get_packet_size(&ring_buf);
            }
        }
    }

    Ok(())
}