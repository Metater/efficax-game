use cgmath::Zero;

use crate::server::constants::ServerConstants;

use super::ServerState;

impl ServerState {
    pub fn tick_physics(&mut self, delta_time: f32) {
        let step_delta_time = delta_time / ServerConstants::PHYSICS_SUBSTEPS as f32;

        for client in self.clients.values_mut() {
            client.cache_movement_force();
        }

        for _ in 0..ServerConstants::PHYSICS_SUBSTEPS {
            self.substep_entity(step_delta_time);

            self.zone.tick(self.tick_id, step_delta_time);
        }
    }

    fn substep_entity(&mut self, step_delta_time: f32) {
        for client in self.clients.values() {
            if !client.movement_force.is_zero() {
                if let Some(entity) = self.zone.entities.get_mut(&client.id) {
                    entity.add_force(client.movement_force, step_delta_time);
                }
            }
        }
    }
}