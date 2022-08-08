use std::net::SocketAddr;

use cgmath::Vector2;

use crate::{network::data::{NetworkData, SnapshotData, EntitySnapshotData, types::{PositionData, EntityType}, EntitySpecificSnapshotData, PlayerSnapshotData, JoinData, SpawnData}, server::state::ServerState};

impl ServerState {
    pub fn notify_new_player_of_new_player(&self, addr: SocketAddr, player_id: u32, player_pos: Vector2<f32>) {
        let data = NetworkData::Join(JoinData {
            player_id,
            pos: PositionData::new(player_pos)
        });
        self.net.unicast(true, addr, self.tick_id, data);
    }
    pub fn notify_new_player_of_existing_players(&self, addr: SocketAddr, player_id: u32) {
        for client in self.clients.values() {
            if client.id == player_id {
                continue;
            }

            if let Some(entity) = self.zone.entities.get(&client.id) {
                let data = NetworkData::Spawn(SpawnData {
                    entity_type: EntityType::Player,
                    entity_id: client.id,
                    pos: PositionData::new(entity.pos),
                });
                self.net.unicast(true, addr, self.tick_id, data);
            }
        }
    }
    pub fn notify_existing_players_of_new_player(&self, addr: SocketAddr, player_id: u32, player_pos: Vector2<f32>) {
        let data = NetworkData::Spawn(SpawnData {
            entity_type: EntityType::Player,
            entity_id: player_id,
            pos: PositionData::new(player_pos),
        });

        let mut addrs = self.get_client_addrs();

        // skip new player's address
        addrs.remove(addrs.iter().position(|e| *e == addr).unwrap());

        if addrs.len() != 0 {
            self.net.multicast(true, addrs, self.tick_id, data);
        }
    }

    pub fn tick_net_out(&mut self) {
        if self.clients.len() == 0 {
            return
        }

        let mut entity_snapshots = Vec::new();

        for client in self.clients.values() {
            if let Some(entity) = self.zone.entities.get(&client.id) {
                let snapshot = EntitySnapshotData {
                    id: entity.id,
                    pos: PositionData::new(entity.pos),
                    data: EntitySpecificSnapshotData::Player({
                        PlayerSnapshotData {
                            input_sequence: client.input_sequence
                        }
                    })
                };
                entity_snapshots.push(snapshot);
            }
        }

        self.net.multicast(false, self.get_client_addrs(), self.tick_id, NetworkData::Snapshot(SnapshotData {
            entity_snapshots
        }));
    }
}