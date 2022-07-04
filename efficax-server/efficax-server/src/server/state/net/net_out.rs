use std::net::SocketAddr;

use crate::{network::data::{NetworkData, SnapshotData, EntitySnapshotData, types::PositionData, EntitySpecificSnapshotData, PlayerSnapshotData}, server::state::ServerState};

impl ServerState {
    pub fn tick_net_out(&mut self) {
        if self.clients.len() == 0 {
            return
        }

        let mut entity_snapshots = Vec::new();

        for player in self.clients.values() {
            if let Some(entity) = self.zone.entities.get(&player.id) {
                let snapshot = EntitySnapshotData {
                    id: entity.id,
                    pos: PositionData::new(entity.pos),
                    data: EntitySpecificSnapshotData::Player({
                        PlayerSnapshotData {
                            input_sequence: player.input_sequence
                        }
                    })
                };
                entity_snapshots.push(snapshot);
            }
        }

        let addrs: Vec<SocketAddr> = self.clients.keys().copied().collect();
        self.net.multicast(false, addrs, self.tick_id, NetworkData::Snapshot(SnapshotData {
            entity_snapshots
        }));
    }
}