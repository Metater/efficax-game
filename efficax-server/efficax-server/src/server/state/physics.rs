use super::ServerState;

impl ServerState {
    const PHYSICS_SUBSTEPS: i32 = 8;

    pub fn tick_physics(&mut self, delta_time: f32) {
        let step_delta_time = delta_time / ServerState::PHYSICS_SUBSTEPS as f32;
        for _ in 0..ServerState::PHYSICS_SUBSTEPS {
            self.update_clients(step_delta_time);
            self.zone.tick(self.tick_id, step_delta_time);
        }
    }
}