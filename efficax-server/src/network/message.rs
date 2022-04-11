use std::{net::SocketAddr};

use crate::network::packet::NetworkPacket;
use crate::network::client::NetworkClient;

pub enum NetworkMessage {
    Join(NetworkClient),
    Data(NetworkPacket),
    Leave(SocketAddr)
}