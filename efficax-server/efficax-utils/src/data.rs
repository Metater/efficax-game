use tokio::io::{/*AsyncReadExt, */AsyncWriteExt};

use cgmath::Vector2;

use std::io;

use crate::scaling::scale_f32_as_u16;

pub async fn write_pos(buf: &mut Vec<u8>, pos: Vector2<f32>) -> io::Result<()> {
    buf.write_u16_le(scale_f32_as_u16(-256.0, 256.0, pos.x)).await?;
    buf.write_u16_le(scale_f32_as_u16(-256.0, 256.0, pos.y)).await?;
    Ok(())
}