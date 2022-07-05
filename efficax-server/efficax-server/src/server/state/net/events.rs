use std::net::SocketAddr;

use cgmath::Vector2;
use metaitus::collider::MetaitusCollider;

use crate::{network::data::{NetworkData, DespawnData}, server::{state::{ServerState, client_state::ClientState}, constants::ServerConstants}};

impl ServerState {
    pub fn join(&mut self, addr: SocketAddr) {
        let entity = self.zone.spawn_entity(Vector2::new(0.0, 5.0));
        
        entity
        .with_bounds(true, MetaitusCollider::new(Vector2::new(-15.75, -7.75), Vector2::new(15.25, 7.25)))
        .with_drag(true, 5.0)
        .with_collider(true, MetaitusCollider::new(Vector2::new(-ServerConstants::PLAYER_COLLIDER_RADIUS, -ServerConstants::PLAYER_COLLIDER_RADIUS), Vector2::new(ServerConstants::PLAYER_COLLIDER_RADIUS, ServerConstants::PLAYER_COLLIDER_RADIUS)))
        .with_repulsion_radius(true, 0.4, 48.0, 3.0);
        
        let player_id = entity.id;
        let player_pos = entity.pos;

        self.clients.insert(addr, ClientState::new(player_id));

        self.notify_new_player_of_new_player(addr, player_id, player_pos);
        self.notify_new_player_of_existing_players(addr, player_id);
        self.notify_existing_players_of_new_player(addr, player_id, player_pos);
    }
    
    pub fn leave(&mut self, addr: SocketAddr) {
        if let Some(client) = self.clients.remove(&addr) {
            self.zone.despawn_entity(client.id);

            let data = NetworkData::Despawn(DespawnData {
                entity_id: client.id,
            });
            let addrs = self.get_client_addrs();
            if addrs.len() != 0 {
                self.net.multicast(true, addrs, self.tick_id, data);
            }
        }
    }
}