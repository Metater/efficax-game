use cgmath::{Vector2};

use crate::network::data::input::InputData;

pub struct ClientState {
    pub id: u32,
    pub pos: Vector2<f64>,

    last_input: u8,
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
        let mag = 16.0 * (1.0 / 40.0);
        let dia_mag = mag * 0.70710678118;
        match dir {
            0 => (),
            1 => { // Up
                self.pos.y += mag;
            }
            2 => { // Up, Right
                self.pos.x += dia_mag;
                self.pos.y += dia_mag;
            }
            3 => { // Right
                self.pos.x += mag;
            }
            4 => { // Down, Right
                self.pos.x += dia_mag;
                self.pos.y -= dia_mag;
            }
            5 => { // Down
                self.pos.y -= mag;
            }
            6 => { // Down, Left
                self.pos.x -= dia_mag;
                self.pos.y -= dia_mag;
            }
            7 => { // Left
                self.pos.x -= mag;
            }
            8 => { // Up, Left
                self.pos.x -= dia_mag;
                self.pos.y += dia_mag;
            }
            _ => ()
        }
    }
}