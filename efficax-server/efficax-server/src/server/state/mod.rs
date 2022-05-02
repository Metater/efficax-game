pub mod client_state;

use std::{net::SocketAddr, collections::HashMap};

use cgmath::{Vector2, Zero};
use tokio::sync::mpsc::UnboundedSender;

use crate::network::{data::{entity_update::EntityUpdateData, NetworkData, tick_update::TickUpdateData}, message::NetworkSenderMessage, packet::NetworkPacket};

use self::client_state::ClientState;

use metaitus::{zone::MetaitusZone, physics::collider::PhysicsCollider};

pub struct ServerState {
    pub tick_id: u64,

    pub clients: HashMap<SocketAddr, ClientState>,
    pub zone: MetaitusZone,
}

impl ServerState {
    const METAITUS_SUBSTEPS: i32 = 8;

    pub fn new() -> Self {
        ServerState {
            tick_id: 0,

            clients: HashMap::new(),
            zone: MetaitusZone::new(),
        }
    }

    pub fn tick(&mut self, delta_time: f32, sender_tx: &mut UnboundedSender<NetworkSenderMessage>) {
        //println!("[server state]: tick: {}", self.tick_id);

        // later optimize by only doing lookups for entities once
        let step_delta_time = delta_time / ServerState::METAITUS_SUBSTEPS as f32;
        for _ in 0..ServerState::METAITUS_SUBSTEPS {
            self.update_clients(step_delta_time);
            self.zone.tick(self.tick_id, step_delta_time);
        }
        self.send_client_updates(sender_tx);

        self.tick_id += 1;
    }
}

impl ServerState {
    fn update_clients(&mut self, delta_time: f32) {
        for player in self.clients.values() {
            let movement_force = player.get_movement_force();
            if !movement_force.is_zero() {
                if let Some(entity) = self.zone.entities.get_mut(&player.id) {
                    entity.add_force(movement_force, delta_time);
                }
            }
        }
    }
    fn send_client_updates(&mut self, sender_tx: &mut UnboundedSender<NetworkSenderMessage>) {
        let mut entity_updates = Vec::new();

        for player in self.clients.values() {
            if let Some(entity) = self.zone.entities.get(&player.id) {
                if entity.last_moved_on_tick == self.tick_id {
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
        
        entity
        .with_bounds(true, PhysicsCollider::new(Vector2::new(-5.0, -3.0), Vector2::new(5.0, 3.0)))
        .with_drag(true, true, 3.0)
        .with_collider(true, PhysicsCollider::new(Vector2::new(-0.475, -0.475), Vector2::new(0.475, 0.475)))
        .with_repulsion_radius(true, 0.4, 48.0, 3.0);

        self.clients.insert(addr, ClientState::new(entity.id));
    }
}