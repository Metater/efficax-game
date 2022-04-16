pub mod client_state;

use std::{net::SocketAddr, collections::HashMap};

use cgmath::{Vector2, Zero};
use tokio::sync::mpsc::UnboundedSender;

use crate::network::{data::{entity_update::EntityUpdateData, NetworkData, tick_update::TickUpdateData}, message::NetworkSenderMessage, packet::NetworkPacket};

use self::client_state::ClientState;

use super::{metaitus::zone::MetaitusZone, EfficaxServer};

pub struct ServerState {
    pub tick_id: u64,

    pub clients: HashMap<SocketAddr, ClientState>,
    pub zone: MetaitusZone,
}

impl ServerState {
    pub fn new() -> Self {
        ServerState {
            tick_id: 0,

            clients: HashMap::new(),
            zone: MetaitusZone::new(),
        }
    }

    pub fn tick(&mut self, delta_time: f32, sender_tx: &mut UnboundedSender<NetworkSenderMessage>) {
        // later optimize by only doing lookups for entities once
        self.update_clients(delta_time);
        self.zone.tick(delta_time);
        self.send_client_updates(sender_tx);

        self.tick_id += 1;
    }
}

impl ServerState {
    fn update_clients(&mut self, delta_time: f32) {
        for player in self.clients.values() {
            let movement_force = player.get_movement_force();
            if !movement_force.is_zero() {
                if let Some(entity) = self.zone.get_entity(player.id) {
                    entity.add_force(movement_force, delta_time);
                }
            }
        }
    }
    fn send_client_updates(&mut self, sender_tx: &mut UnboundedSender<NetworkSenderMessage>) {
        let mut entity_updates = Vec::new();

        for player in self.clients.values() {
            if let Some(entity) = self.zone.get_entity(player.id) {
                if entity.moved_xy || entity.tick_count == 1 {
                    let update = EntityUpdateData {
                        id: entity.id,
                        pos: entity.pos,
                        input_sequence: player.input_sequence,
                    };
                    entity_updates.push(update);
                }
            }
        }

        let addrs = self.clients.keys().copied().collect();
        sender_tx.send(NetworkSenderMessage::Data(
            NetworkPacket::new(addrs, NetworkData::TickUpdate(TickUpdateData {
                entity_updates
            }))
        )).ok();
    }
}

impl ServerState {
    /*
    pub fn get_addrs(&self) -> Vec<SocketAddr> {
        self.clients.keys().copied().collect()
    }
    pub fn get_clients(&self) -> Vec<&ClientState> {
        self.clients.values().collect()
    }
    pub fn get_clients_mut(&mut self) -> Vec<&mut ClientState> {
        self.clients.values_mut().collect()
    }
    */
    pub fn get_client(&mut self, addr: &SocketAddr) -> Option<&mut ClientState> {
        self.clients.get_mut(addr)
    }
    pub fn new_client(&mut self, addr: SocketAddr) {
        let entity = self.zone.spawn_entity(Vector2::zero());
        self.clients.insert(addr, ClientState::new(entity.id));
    }
}