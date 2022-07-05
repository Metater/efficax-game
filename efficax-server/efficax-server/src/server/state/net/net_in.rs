use std::net::SocketAddr;

use crate::{network::data::{InputData}, server::state::ServerState};

impl ServerState {
    pub fn input_data(&mut self, addr: SocketAddr, data: &InputData) {
        if let Some(client) = self.clients.get_mut(&addr) {
            client.feed_input(data);
        }
    }
}