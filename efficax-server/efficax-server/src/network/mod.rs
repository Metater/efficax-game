pub mod packet;
pub mod data;

mod receiver;
mod sender;

use std::net::SocketAddr;

use tokio::{sync::mpsc::{self, UnboundedReceiver, UnboundedSender}, net::tcp::OwnedWriteHalf};

use self::{sender::NetworkSender, listener::NetworkListener, data::NetworkData, packet::NetworkPacket};

pub enum NetworkReceiverMessage {
    Join(SocketAddr),
    Leave(SocketAddr),
    Data(NetworkPacket)
}

pub enum NetworkSenderMessage {
    Join((SocketAddr, OwnedWriteHalf)),
    Leave(SocketAddr),
    Data(NetworkPacket)
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
        self.sender_tx.send(NetworkSenderMessage::Data(packet));
    }
}

pub struct EfficaxNetwork {
    listener: NetworkListener,
    sender: NetworkSender
}

impl EfficaxNetwork {
    pub async fn start() -> (Self, UnboundedReceiver<NetworkReceiverMessage>, UnboundedSender<NetworkSenderMessage>) {
        let (listener_tx, listener_rx) = mpsc::unbounded_channel::<NetworkReceiverMessage>();
        let (mut sender_tx, sender_rx) = mpsc::unbounded_channel::<NetworkSenderMessage>();

        let listener = NetworkListener::start(listener_tx, &mut sender_tx).await;
        let sender = NetworkSender::start(sender_rx).await;
        
        let network = EfficaxNetwork {
            listener,
            sender
        };

        (network, listener_rx, sender_tx)
    }

    pub fn stop(&self) {
        self.listener.stop();
        self.sender.stop();
    }
}