pub mod packet;
pub mod data;

mod receiver;
mod sender;

use std::{net::SocketAddr, sync::Arc};

use tokio::{sync::{mpsc::{self, UnboundedReceiver, UnboundedSender}, Notify}, net::tcp::OwnedWriteHalf, task::JoinHandle};

use self::{data::NetworkData, packet::NetworkPacket};

pub enum NetworkReceiverMessage {
    Join(SocketAddr),
    Leave(SocketAddr),
    Data(NetworkPacket)
}

pub enum NetworkSenderMessage {
    Join((SocketAddr, OwnedWriteHalf)),
    Leave(SocketAddr),
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

    pub fn get_new_handle(&self) -> Self {
        Self::new(self.sender_tx.clone())
    }

    pub fn unicast(&self, addr: SocketAddr, data: NetworkData) {
        self.send(NetworkPacket::unicast(addr, data));
    }

    pub fn multicast(&self, addrs: Vec<SocketAddr>, data: NetworkData) {
        self.send(NetworkPacket::multicast(addrs, data));
    }

    pub fn send(&self, packet: NetworkPacket) {
        self.sender_tx.send(NetworkSenderMessage::Data(packet)).ok();
    }
}

pub async fn start() -> (UnboundedReceiver<NetworkReceiverMessage>, UnboundedSender<NetworkSenderMessage>, Arc<Notify>, JoinHandle<()>, JoinHandle<()>) {
    let (receiver_tx, receiver_rx) = mpsc::unbounded_channel::<NetworkReceiverMessage>();
    let (sender_tx, sender_rx) = mpsc::unbounded_channel::<NetworkSenderMessage>();

    let (receiver_stop_notifier, receiver_handle) = receiver::start(receiver_tx, sender_tx.clone()).await;
    let sender_handle = sender::start(sender_rx).await;

    (receiver_rx, sender_tx, receiver_stop_notifier, receiver_handle, sender_handle)
}