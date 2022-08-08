use std::net::SocketAddr;

use tokio::sync::mpsc::UnboundedSender;

use super::{NetworkSenderMessage, packet::NetworkPacket, data::NetworkData};

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

    pub fn unicast(&self, is_tcp: bool, addr: SocketAddr, tick_id: u32, data: NetworkData) {
        self.send(NetworkPacket::unicast(is_tcp, addr, tick_id, data));
    }

    pub fn multicast(&self, is_tcp: bool, addrs: Vec<SocketAddr>, tick_id: u32, data: NetworkData) {
        self.send(NetworkPacket::multicast(is_tcp, addrs, tick_id, data));
    }

    fn send(&self, packet: NetworkPacket) {
        self.sender_tx.send(NetworkSenderMessage::Data(packet)).expect("failed to send packet");
    }
}