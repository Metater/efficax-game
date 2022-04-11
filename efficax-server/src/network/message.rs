use std::{net::SocketAddr};

use crate::network::client::NetworkClient;

use super::packet::NetworkPacket;

pub enum NetworkMessage {
    Join(NetworkClient),
    Data(NetworkPacket),
    Leave(SocketAddr)
}