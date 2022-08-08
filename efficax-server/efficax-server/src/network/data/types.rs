use cgmath::Vector2;
use efficax_utils::scaling::{scale_f32_as_u16, unscale_u16_as_f32};

// Position
#[derive(bincode::Encode, bincode::Decode, Copy, Clone, Debug)]
pub struct PositionData {
    x: u16,
    y: u16,
}

impl PositionData {
    pub fn new(pos: Vector2<f32>) -> Self {
        PositionData {
            x: scale_f32_as_u16(-256.0, 256.0, pos.x),
            y: scale_f32_as_u16(-256.0, 256.0, pos.y)
        }
    }

    pub fn _get(&self) -> Vector2<f32> {
        Vector2::new(unscale_u16_as_f32(-256.0, 256.0, self.x), unscale_u16_as_f32(-256.0, 256.0, self.y))
    }
}

// EntityType
#[derive(Copy, Clone, Debug)]
pub enum EntityType {
    Player
}