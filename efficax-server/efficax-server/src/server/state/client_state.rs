use std::f64::consts::PI;

use cgmath::{Vector2, Zero};

use efficax_utils::scaling;

use crate::network::data::InputData;

pub struct ClientState {
    pub id: u32,

    pub input: u8,
    pub input_sequence: u8,

    pub movement_force: Vector2<f32>
}

impl ClientState {
    pub fn new(id: u32) -> Self {
        ClientState {
            id,

            input: 0,
            input_sequence: 0,

            movement_force: Vector2::zero()
        }
    }

    pub fn feed_input(&mut self, data: &InputData) {
        let greater_or_equal = data.input_sequence >= self.input_sequence;
        let wraps = data.input_sequence < 63 && self.input_sequence > 127;
        if greater_or_equal || wraps {
            self.input = data.input % 9;
            self.input_sequence = data.input_sequence;
        }
    }

    pub fn cache_movement_force(&mut self) {
        let dir = self.input;
        
        if dir == 0 {
            self.movement_force = Vector2::zero();
        }
        
        let mag = 40.0;
        let rot = (scaling::linear_step(1.0, 9.0, dir.into()) - 0.25) * -2.0 * PI;
        let x_force  = rot.cos() * mag;
        let y_force = rot.sin() * mag;
        self.movement_force = Vector2::new(x_force as f32, y_force as f32);
    }
}