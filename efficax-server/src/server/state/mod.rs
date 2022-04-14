pub mod client_state;

use std::{net::SocketAddr, collections::HashMap};

use self::client_state::ClientState;

pub struct ServerState {
    pub tick_id: u64,

    next_entity_id: u32,

    pub clients: HashMap<SocketAddr, ClientState>,
}

impl ServerState {
    pub fn new() -> Self {
        ServerState {
            tick_id: 0,

            next_entity_id: 0,

            clients: HashMap::new(),
        }
    }

    pub fn tick(&mut self) {
        self.tick_id += 1;
    }

    pub fn get_next_entity_id(&mut self) -> u32 {
        let entity_id = self.next_entity_id;
        self.next_entity_id += 1;
        entity_id
    }
}

impl ServerState {
    pub fn get_addrs(&self) -> Vec<SocketAddr> {
        self.clients.keys().copied().collect()
    }
    pub fn get_clients(&self) -> Vec<&ClientState> {
        self.clients.values().collect()
    }
    pub fn get_clients_mut(&mut self) -> Vec<&mut ClientState> {
        self.clients.values_mut().collect()
    }
    pub fn get_client(&mut self, addr: &SocketAddr) -> Option<&mut ClientState> {
        self.clients.get_mut(addr)
    }
    pub fn new_client(&mut self, addr: SocketAddr) {
        let entity_id = self.get_next_entity_id();
        self.clients.insert(addr, ClientState::new(entity_id));
    }
}