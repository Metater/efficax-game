use cgmath::Vector2;

#[derive(bincode::Encode, bincode::Decode, Debug)]
pub struct Vector2f32Data {
    pub x: f32,
    pub y: f32,
}

impl Vector2f32Data {
    pub fn new(vec: Vector2<f32>) -> Self {
        Vector2f32Data {
            x: vec.x,
            y: vec.y
        }
    }
}