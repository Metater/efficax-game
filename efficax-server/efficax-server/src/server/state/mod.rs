pub mod client_state;

use std::{net::SocketAddr, collections::HashMap};

use cgmath::{Vector2, Zero};
use tokio::sync::mpsc::UnboundedSender;

use metaitus::{zone::MetaitusZone, collider::MetaitusCollider};

use crate::network::{NetworkSenderHandle, NetworkSenderMessage, data::{EntityUpdateData, TickUpdateData, NetworkData, InputData, types::PositionData}};

use self::client_state::ClientState;

pub struct ServerState {
    pub tick_id: u64,

    pub clients: HashMap<SocketAddr, ClientState>,

    zone: MetaitusZone,

    net: NetworkSenderHandle
}

impl ServerState {
    const PHYSICS_SUBSTEPS: i32 = 8;

    pub fn new(sender_tx: UnboundedSender<NetworkSenderMessage>) -> Self {
        ServerState {
            tick_id: 0,

            clients: HashMap::new(),

            zone: MetaitusZone::new(),

            net: NetworkSenderHandle::new(sender_tx)
        }
    }

    pub fn tick(&mut self, delta_time: f32) {
        //println!("[server state]: tick: {}", self.tick_id);

        // later optimize by only doing lookups for entities once per tick
        let step_delta_time = delta_time / ServerState::PHYSICS_SUBSTEPS as f32;
        for _ in 0..ServerState::PHYSICS_SUBSTEPS {
            self.update_clients(step_delta_time);
            self.zone.tick(self.tick_id, step_delta_time);
        }

        self.send_client_updates();

        self.tick_id += 1;
    }
}

impl ServerState {
    fn update_clients(&mut self, delta_time: f32) {
        for player in self.clients.values() {
            // continual lookup of movement force when it wont change
            // movement force is constant for the duration of the tick
            let movement_force = player.get_movement_force();
            if !movement_force.is_zero() {
                if let Some(entity) = self.zone.entities.get_mut(&player.id) {
                    entity.add_force(movement_force, delta_time);
                }
            }
        }
    }
    
    fn send_client_updates(&mut self) {
        let mut entity_updates = Vec::new();

        for player in self.clients.values() {
            if let Some(entity) = self.zone.entities.get(&player.id) {
                //if entity.last_moved_on_tick == self.tick_id || entity.last_moved_on_tick == 0 {
                let update = EntityUpdateData {
                    id: entity.id,
                    pos: PositionData::new(entity.pos),
                    input_sequence: player.input_sequence,
                };
                entity_updates.push(update);
                //}
            }
        }

        let addrs: Vec<SocketAddr> = self.clients.keys().copied().collect();
        if addrs.len() > 0 { // if there are clients, REMOVE IF OTHER STUFF
            self.net.multicast(false, addrs, NetworkData::TickUpdate(TickUpdateData {
                entity_updates
            }));
        }
    }
}

impl ServerState {
    pub fn join(&mut self, addr: SocketAddr) {
        let entity = self.zone.spawn_entity(Vector2::zero());
        
        entity
        .with_bounds(true, MetaitusCollider::new(Vector2::new(-5.0, -3.0), Vector2::new(5.0, 3.0)))
        .with_drag(true, 3.0)
        .with_collider(true, MetaitusCollider::new(Vector2::new(-0.475, -0.475), Vector2::new(0.475, 0.475)))
        .with_repulsion_radius(true, 0.4, 48.0, 3.0);

        self.clients.insert(addr, ClientState::new(entity.id));
    }
    pub fn leave(&mut self, addr: SocketAddr) {
        if let Some(client) = self.clients.remove(&addr) {
            self.zone.despawn_entity(client.id);
        }
    }

    pub fn input_data(&mut self, addr: SocketAddr, data: &InputData) {
        if let Some(player) = self.clients.get_mut(&addr) {
            player.feed_input(data);
        }
    }
}