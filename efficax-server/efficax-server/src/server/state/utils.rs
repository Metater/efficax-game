use std::net::SocketAddr;

use super::ServerState;

impl ServerState {
    pub fn get_client_addrs(&self) -> Vec<SocketAddr> {
        self.clients.keys().copied().collect()
    }
}