use std::f64::consts::PI;

use cgmath::{Vector2, Zero};

use crate::network::data::input::InputData;
use crate::utils;

pub struct ClientState {
    pub id: u32,

    pub last_input: u8,
    pub input_sequence: u8,
}

impl ClientState {
    pub fn new(id: u32) -> Self {
        ClientState {
            id,

            last_input: 0,
            input_sequence: 0
        }
    }

    pub fn feed_input(&mut self, data: &InputData) {
        self.last_input = data.input % 9;
        self.input_sequence = data.input_sequence;
    }

    pub fn get_movement_force(&self) -> Vector2<f32> {
        let dir = self.last_input;
        
        if dir == 0 {
            return Vector2::zero();
        }
        
        let mag = 32.0;
        let rot = (utils::linear_step(1.0, 9.0, dir.into()) - 0.25) * -2.0 * PI;
        let x_force  = rot.cos() * mag;
        let y_force = rot.sin() * mag;
        return Vector2::new(x_force as f32, y_force as f32);
    }
}