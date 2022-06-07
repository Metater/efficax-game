pub mod packet;
pub mod data;

mod receiver;
mod sender;

use std::{net::SocketAddr, sync::Arc};

use tokio::{sync::{mpsc::{self, UnboundedReceiver, UnboundedSender}, Notify}, net::tcp::OwnedWriteHalf, task::JoinHandle};

use self::{data::NetworkData, packet::NetworkPacket};

#[derive(Debug)]
pub struct NetworkClient {
    addr: SocketAddr,
    writer: OwnedWriteHalf,
    udp_port: u16
}

impl NetworkClient {
    pub fn new(addr: SocketAddr, writer: OwnedWriteHalf) -> Self {
        Self {
            addr,
            writer,
            udp_port: 0
        }
    }
}

#[derive(Debug)]
pub enum NetworkReceiverMessage {
    Join(SocketAddr),
    Leave(SocketAddr),
    InitUDP((SocketAddr, u16)),
    Data(NetworkPacket)
}

#[derive(Debug)]
pub enum NetworkSenderMessage {
    Join(NetworkClient),
    Leave(SocketAddr),
    InitUDP((SocketAddr, u16)),
    Data(NetworkPacket),
    Stop
}

pub struct NetworkSenderHandle {
    sender_tx: UnboundedSender<NetworkSenderMessage>
}

impl NetworkSenderHandle {
    pub fn new(sender_tx: UnboundedSender<NetworkSenderMessage>) -> Self {
        NetworkSenderHandle {
            sender_tx
        }
    }

    pub fn _get_new_handle(&self) -> Self {
        Self::new(self.sender_tx.clone())
    }

    pub fn unicast(&self, is_tcp: bool, addr: SocketAddr, tick_id: u8, data: NetworkData) {
        self.send(NetworkPacket::unicast(is_tcp, addr, tick_id, data));
    }

    pub fn multicast(&self, is_tcp: bool, addrs: Vec<SocketAddr>, tick_id: u8, data: NetworkData) {
        self.send(NetworkPacket::multicast(is_tcp, addrs, tick_id, data));
    }

    fn send(&self, packet: NetworkPacket) {
        self.sender_tx.send(NetworkSenderMessage::Data(packet)).expect("failed to send packet");
    }
}

pub async fn start() -> (UnboundedReceiver<NetworkReceiverMessage>, UnboundedSender<NetworkSenderMessage>, Arc<Notify>, JoinHandle<()>, JoinHandle<()>, JoinHandle<()>) {
    let (receiver_tx, receiver_rx) = mpsc::unbounded_channel::<NetworkReceiverMessage>();
    let (sender_tx, sender_rx) = mpsc::unbounded_channel::<NetworkSenderMessage>();

    let (udp_socket, receiver_stop_notifier, receiver_handle, udp_receiver_handle) = receiver::start(receiver_tx, sender_tx.clone()).await;
    let sender_handle = sender::start(sender_rx, udp_socket).await;

    (receiver_rx, sender_tx, receiver_stop_notifier, receiver_handle, udp_receiver_handle, sender_handle)
}