use std::f64::consts::PI;

use cgmath::{Vector2};

use crate::network::data::input::InputData;
use crate::utils;

pub struct ClientState {
    pub id: u32,
    pub pos: Vector2<f64>,
    pub last_input: u8,
}

impl ClientState {
    pub fn new(id: u32) -> Self {
        ClientState {
            id,
            pos: Vector2::new(0.0, 0.0),

            last_input: 0,
        }
    }

    pub fn feed_input(&mut self, data: &InputData) {
        self.last_input = data.input % 9;
    }

    pub fn apply_input(&mut self) {
        let dir = self.last_input;
        
        if dir == 0 {
            return
        }
        
        let mag = 16.0 * (1.0 / 40.0);
        let rot = (utils::linear_step(1.0, 9.0, dir.into()) - 0.25) * -2.0 * PI;
        self.pos.x += rot.cos() * mag;
        self.pos.y += rot.sin() * mag;
    }
}