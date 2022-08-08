pub mod packet;
pub mod data;
pub mod sender_handle;

mod receiver;
mod sender;

use std::{net::SocketAddr, sync::Arc};

use tokio::{sync::{mpsc::{self, UnboundedReceiver, UnboundedSender}, Notify}, net::tcp::OwnedWriteHalf, task::JoinHandle};

use self::packet::NetworkPacket;

// NetworkClient
#[derive(Debug)]
pub struct NetworkClient {
    addr: SocketAddr,
    writer: OwnedWriteHalf,
    remote_udp_port: u16
}

impl NetworkClient {
    pub fn new(addr: SocketAddr, writer: OwnedWriteHalf) -> Self {
        Self {
            addr,
            writer,
            remote_udp_port: 0
        }
    }
}

// NetworkReceiverMessage
#[derive(Debug)]
pub enum NetworkReceiverMessage {
    Join(SocketAddr),
    Leave(SocketAddr),
    InitNetwork((SocketAddr, u16)),
    Data(NetworkPacket)
}

// NetworkSenderMessage
#[derive(Debug)]
pub enum NetworkSenderMessage {
    Join(NetworkClient),
    Leave(SocketAddr),
    InitNetwork((SocketAddr, u16)),
    Data(NetworkPacket),
    Stop
}

pub async fn start() -> (UnboundedReceiver<NetworkReceiverMessage>, UnboundedSender<NetworkSenderMessage>, Arc<Notify>,JoinHandle<()>, JoinHandle<()>, JoinHandle<()>) {
    let (receiver_tx, receiver_rx) = mpsc::unbounded_channel::<NetworkReceiverMessage>();
    let (sender_tx, sender_rx) = mpsc::unbounded_channel::<NetworkSenderMessage>();

    let (udp_socket, receiver_stop_notifier, receiver_handle, udp_receiver_handle) = receiver::start(receiver_tx, sender_tx.clone()).await;
    let sender_handle = sender::start(sender_rx, udp_socket).await;

    (receiver_rx, sender_tx, receiver_stop_notifier, receiver_handle, udp_receiver_handle, sender_handle)
}