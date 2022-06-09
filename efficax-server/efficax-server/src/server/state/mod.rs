pub mod client_state;
pub mod physics;

use std::{net::SocketAddr, collections::HashMap};

use cgmath::{Vector2, Zero};
use tokio::sync::mpsc::UnboundedSender;

use metaitus::{zone::MetaitusZone, collider::MetaitusCollider, entity::MetaitusEntity};

use crate::network::{NetworkSenderHandle, NetworkSenderMessage, data::{EntitySnapshotData, SnapshotData, NetworkData, InputData, types::PositionData, EntitySpecificSnapshotData, PlayerSnapshotData, JoinData}};

use self::client_state::ClientState;

pub struct ServerState {
    pub tick_id: u64,

    pub clients: HashMap<SocketAddr, ClientState>,

    zone: MetaitusZone,

    net: NetworkSenderHandle,
}

impl ServerState {
    pub fn new(sender_tx: UnboundedSender<NetworkSenderMessage>) -> Self {
        ServerState {
            tick_id: 0,

            clients: HashMap::new(),

            zone: MetaitusZone::new(),

            net: NetworkSenderHandle::new(sender_tx),
        }
    }

    pub fn init(&mut self) {

    }

    pub fn tick(&mut self, delta_time: f32) {
        //println!("[server state]: tick: {}", self.tick_id);

        self.tick_physics();

        // later optimize by only doing lookups for entities once per tick

        self.send_client_updates(delta_time);

        self.tick_id += 1;
    }

    pub fn get_tick_id_u8(&self) -> u8 {
        (self.tick_id % 256) as u8
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
        if self.clients.len() == 0 {
            return
        }

        let mut entity_snapshots = Vec::new();

        for player in self.clients.values() {
            if let Some(entity) = self.zone.entities.get(&player.id) {
                Self::add_snapshot(&mut entity_snapshots, entity, player.input_sequence);
            }
        }

        let addrs: Vec<SocketAddr> = self.clients.keys().copied().collect();
        self.net.multicast(false, addrs, self.get_tick_id_u8(), NetworkData::Snapshot(SnapshotData {
            entity_snapshots
        }));
    }

    fn add_snapshot(entity_snapshots: &mut Vec<EntitySnapshotData>, entity: &MetaitusEntity, input_sequence: u8) {
        let snapshot = EntitySnapshotData {
            id: entity.id,
            pos: PositionData::new(entity.pos),
            data: EntitySpecificSnapshotData::Player({
                PlayerSnapshotData {
                    input_sequence
                }
            })
        };
        entity_snapshots.push(snapshot);
    }
}

impl ServerState {
    pub fn join(&mut self, addr: SocketAddr) {
        let entity = self.zone.spawn_entity(Vector2::zero());
        
        entity
        .with_bounds(true, MetaitusCollider::new(Vector2::new(-5.0, -3.0), Vector2::new(5.0, 3.0)))
        .with_drag(true, 5.0)
        .with_collider(true, MetaitusCollider::new(Vector2::new(-0.475, -0.475), Vector2::new(0.475, 0.475)))
        .with_repulsion_radius(true, 0.4, 48.0, 3.0);
        
        self.clients.insert(addr, ClientState::new(entity.id));

        // Send join packet
        let data = NetworkData::Join(JoinData {
            player_id: entity.id
        });
        self.net.unicast(true, addr, self.get_tick_id_u8(), data);
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