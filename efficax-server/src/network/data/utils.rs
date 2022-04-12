use tokio::io::{AsyncReadExt, AsyncWriteExt};

use std::io;

use cgmath::Point2;

pub async fn write_pos(buf: &mut Vec<u8>, pos: Point2<f64>) -> io::Result<()> {
    buf.write_u16(scale_f64_as_u16(-256.0, 256.0, pos.x)).await?;
    buf.write_u16(scale_f64_as_u16(-256.0, 256.0, pos.y)).await?;
    Ok(())
}

pub fn scale_f32_as_u16(lower: f32, upper: f32, value: f32) -> u16 {
    let step = linear_step(lower.into(), upper.into(), value.into());
    lerp(0.0, 65535.0, step).round() as u16
}
pub fn scale_f64_as_u16(lower: f64, upper: f64, value: f64) -> u16 {
    let step = linear_step(lower, upper, value);
    lerp(0.0, 65535.0, step).round() as u16
}

pub fn unscale_u16_as_f32(lower: f32, upper: f32, value: u16) -> f32 {
    let step = linear_step(0.0, 65535.0, value.into());
    lerp(lower.into(), upper.into(), step) as f32
}
pub fn unscale_u16_as_f64(lower: f64, upper: f64, value: u16) -> f64 {
    let step = linear_step(0.0, 65535.0, value.into());
    lerp(lower, upper, step)
}

pub fn scale_f32_as_u8(lower: f32, upper: f32, value: f32) -> u8 {
    let step = linear_step(lower.into(), upper.into(), value.into());
    lerp(0.0, 255.0, step).round() as u8
}
pub fn scale_f64_as_u8(lower: f64, upper: f64, value: f64) -> u8 {
    let step = linear_step(lower, upper, value);
    lerp(0.0, 255.0, step).round() as u8
}

pub fn unscale_u8_as_f32(lower: f32, upper: f32, value: u8) -> f32 {
    let step = linear_step(0.0, 255.0, value.into());
    lerp(lower.into(), upper.into(), step) as f32
}
pub fn unscale_u8_as_f64(lower: f64, upper: f64, value: u8) -> f64 {
    let step = linear_step(0.0, 255.0, value.into());
    lerp(lower, upper, step)
}

fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + t * (b - a)
}
fn linear_step(a: f64, b: f64, t: f64) -> f64 {
    (t - a) / (b - a)
}