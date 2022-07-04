use std::net::SocketAddr;

use cgmath::{Vector2, Zero};
use metaitus::collider::MetaitusCollider;

use crate::{network::data::{NetworkData, JoinData, types::PositionData}, server::state::{ServerState, client_state::ClientState}};

impl ServerState {
    pub fn join(&mut self, addr: SocketAddr) {
        let entity = self.zone.spawn_entity(Vector2::zero());
        
        entity
        .with_bounds(false, MetaitusCollider::new(Vector2::new(-5.0, -3.0), Vector2::new(5.0, 3.0)))
        .with_drag(true, 5.0)
        .with_collider(true, MetaitusCollider::new(Vector2::new(-0.475, -0.475), Vector2::new(0.475, 0.475)))
        .with_repulsion_radius(true, 0.4, 48.0, 3.0);
        
        self.clients.insert(addr, ClientState::new(entity.id));

        // Send join packet
        let data = NetworkData::Join(JoinData {
            player_id: entity.id,
            pos: PositionData::new(entity.pos)
        });
        self.net.unicast(true, addr, self.tick_id, data);
    }
    pub fn leave(&mut self, addr: SocketAddr) {
        if let Some(client) = self.clients.remove(&addr) {
            self.zone.despawn_entity(client.id);
        }
    }
}