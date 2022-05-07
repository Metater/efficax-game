use cgmath::Vector2;

#[derive(bincode::Encode, bincode::Decode, Debug)]
pub struct Vector2f32Data {
    x: f32,
    y: f32,
}

impl Vector2f32Data {
    pub fn new(vec: Vector2<f32>) -> Self {
        Vector2f32Data {
            x: vec.x,
            y: vec.y
        }
    }

    pub fn get(&self) -> Vector2<f32> {
        Vector2::new(self.x, self.y)
    }
}