use cgmath::Point2;

use crate::network::data::input::InputData;

pub struct PlayerState {
    pub id: u32,
    pub pos: Point2<f64>,

    last_input: u8,
}

impl PlayerState {
    pub fn new(id: u32, pos: Point2<f64>) -> PlayerState {
        PlayerState {
            id,
            pos,

            last_input: 0,
        }
    }

    pub fn feed_input(&self, data: &InputData) {
        self.last_input = data.input % 9;
    }

    pub fn apply_input(&self) {
        let dir = self.last_input;
        let mag = 1.0 * (1.0 / 40.0);
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