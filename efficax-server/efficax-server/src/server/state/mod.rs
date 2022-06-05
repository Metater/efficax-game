pub mod client_state;

use std::{net::SocketAddr, collections::HashMap};

use cgmath::{Vector2, Zero};
use tokio::sync::mpsc::UnboundedSender;

use metaitus::{zone::MetaitusZone, collider::MetaitusCollider, entity::MetaitusEntity};

use crate::network::{NetworkSenderHandle, NetworkSenderMessage, data::{EntitySnapshotData, SnapshotData, NetworkData, InputData, types::PositionData, EntitySpecificSnapshotData, PlayerSnapshotData}};

use self::client_state::ClientState;

pub struct ServerState {
    pub tick_id: u64,

    pub clients: HashMap<SocketAddr, ClientState>,

    zone: MetaitusZone,

    net: NetworkSenderHandle,

    npc: u64
}

impl ServerState {
    const PHYSICS_SUBSTEPS: i32 = 8;

    pub fn new(sender_tx: UnboundedSender<NetworkSenderMessage>) -> Self {
        ServerState {
            tick_id: 0,

            clients: HashMap::new(),

            zone: MetaitusZone::new(),

            net: NetworkSenderHandle::new(sender_tx),

            npc: 0
        }
    }

    pub fn init(&mut self) {
        let npc_entity = self.zone.spawn_entity(Vector2::zero())
            .with_drag(false, 0f32);
        self.npc = npc_entity.id;
    }

    pub fn tick(&mut self, delta_time: f32) {
        //println!("[server state]: tick: {}", self.tick_id);

        // later optimize by only doing lookups for entities once per tick
        let step_delta_time = delta_time / ServerState::PHYSICS_SUBSTEPS as f32;
        for i in 0..ServerState::PHYSICS_SUBSTEPS {
            self.update_clients(step_delta_time);
            // NPC movement
            {
                let time = ((self.tick_id as f64 * 0.04) + ((i as f64) * (step_delta_time as f64))) * 2.0;
                let npc_entity = self.zone.entities.get_mut(&self.npc).unwrap();
                let target_pos = Vector2::new(time.cos() as f32 * 4f32, time.sin() as f32 * 4f32);
                npc_entity.teleport_unchecked(target_pos);
            }
            self.zone.tick(self.tick_id, step_delta_time);
        }

        self.send_client_updates();

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

        Self::add_snapshot(&mut entity_snapshots, self.zone.entities.get(&self.npc).unwrap(), 0);

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