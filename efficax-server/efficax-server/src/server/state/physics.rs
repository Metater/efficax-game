use cgmath::Zero;

use super::ServerState;

impl ServerState {
    const PHYSICS_SUBSTEPS: i32 = 8;

    pub fn tick_physics(&mut self, delta_time: f32) {
        let step_delta_time = delta_time / ServerState::PHYSICS_SUBSTEPS as f32;

        for client in self.clients.values_mut() {
            client.cache_movement_force();
        }

        for _ in 0..ServerState::PHYSICS_SUBSTEPS {
            self.substep_entity(step_delta_time);

            self.zone.tick(self.tick_id, step_delta_time);
        }
    }

    fn substep_entity(&mut self, step_delta_time: f32) {
        for client in self.clients.values() {
            let movement_force = client.movement_force;
            if !movement_force.is_zero() {
                if let Some(entity) = self.zone.entities.get_mut(&client.id) {
                    entity.add_force(movement_force, step_delta_time);
                }
            }
        }
    }
}