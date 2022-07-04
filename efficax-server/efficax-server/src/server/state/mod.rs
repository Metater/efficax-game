pub mod world;

pub mod client_state;
pub mod physics;
pub mod net_out;
pub mod net_in;

use std::{net::SocketAddr, collections::HashMap};

use cgmath::Vector2;
use tokio::sync::mpsc::UnboundedSender;

use metaitus::{zone::MetaitusZone, collider::MetaitusCollider, entity::MetaitusEntity};

use crate::network::{NetworkSenderHandle, NetworkSenderMessage, data::{EntitySnapshotData, SnapshotData, NetworkData, types::PositionData, EntitySpecificSnapshotData, PlayerSnapshotData}};

use self::client_state::ClientState;

pub struct ServerState {
    pub tick_id: u32,

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

            net: NetworkSenderHandle::new(sender_tx)
        }
    }

    pub fn init(&mut self) {
        self.zone.add_static(MetaitusCollider::new(Vector2::new(2.0, 0.0), Vector2::new(3.0, 1.0)));
        self.zone.add_static(MetaitusCollider::new(Vector2::new(4.0, 0.0), Vector2::new(5.0, 1.0)));
    }

    pub fn tick(&mut self, delta_time: f32) {
        //println!("[server state]: tick: {}", self.tick_id);

        self.tick_physics(delta_time);

        // later optimize by only doing lookups for entities once per tick

        self.send_client_updates();

        self.tick_id += 1;
    }
}

impl ServerState {
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
        self.net.multicast(false, addrs, self.tick_id, NetworkData::Snapshot(SnapshotData {
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