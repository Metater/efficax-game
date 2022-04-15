use cgmath::Vector2;
use tokio::io::{AsyncWriteExt};
use std::io;

use crate::utils;

#[derive(Debug)]
pub struct EntityUpdateData {
    pub id: u32,
    pub pos: Vector2<f64>,
    pub input_sequence: u8,
}

impl EntityUpdateData {
    pub async fn write(&self, buf: &mut Vec<u8>) -> io::Result<()> {
        buf.write_u32_le(self.id).await?;
        utils::write_pos(buf, self.pos).await?;
        buf.write_u8(self.input_sequence).await?;
        Ok(())
    }
}