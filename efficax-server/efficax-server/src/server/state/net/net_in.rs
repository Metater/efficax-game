use std::net::SocketAddr;

use crate::{network::{data::{InputData, NetworkData}, packet::NetworkPacket}, server::state::ServerState};

impl ServerState {
    pub fn data(&mut self, packet: NetworkPacket, addr: SocketAddr) {
        match packet.data {
            NetworkData::Input(ref data) => {
                self.input_data(addr, data);
            }
            _ => {
                println!("[server]: client: {} sent unhandleable packet: {:?}", addr, packet.data);
            }
        }
    }

    pub fn input_data(&mut self, addr: SocketAddr, data: &InputData) {
        if let Some(client) = self.clients.get_mut(&addr) {
            client.feed_input(data);
        }
    }
}