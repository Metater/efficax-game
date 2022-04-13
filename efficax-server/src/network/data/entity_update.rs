use cgmath::Vector2;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::io;

use std::io::Cursor;

use super::utils;

#[derive(Debug)]
pub struct EntityUpdateData {
    pub id: u32,
    pub pos: Vector2<f64>
}

impl EntityUpdateData {
    pub const ID: u8 = 2;

    pub fn new(id: u32, pos: Vector2<f64>) -> EntityUpdateData {
        EntityUpdateData {
            id,
            pos
        }
    }

    pub async fn write(&self, buf: &mut Vec<u8>) -> io::Result<()> {
        buf.write_u8(EntityUpdateData::ID).await?;
        buf.write_u32_le(self.id).await?;
        utils::write_pos(buf, self.pos).await?;
        Ok(())
    }
}