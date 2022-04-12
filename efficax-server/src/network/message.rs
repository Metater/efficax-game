use std::{net::SocketAddr};

use tokio::net::tcp::OwnedWriteHalf;

use super::packet::NetworkPacket;

pub enum NetworkListenerMessage {
    Join(SocketAddr),
    Data(NetworkPacket),
    Leave(SocketAddr)
}

pub enum NetworkSenderMessage {
    Join((SocketAddr, OwnedWriteHalf)),
    Data(NetworkPacket),
    Leave(SocketAddr)
}