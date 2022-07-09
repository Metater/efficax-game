x: u16
y: u16

cs-ext
end-cs-ext

rs-ext
pub fn new(pos: cgmath::Vector2<f32>) -> Self {
    Self {
        x: scale_f32_as_u16(-256.0, 256.0, pos.x),
        y: scale_f32_as_u16(-256.0, 256.0, pos.y)
    }
}
end-rs-ext
pub fn get(&self) -> cgmath::Vector2<f32> {
    cgmath::Vector2::new(self.x, self.y)
}
end-rs-ext