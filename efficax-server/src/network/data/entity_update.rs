use cgmath::Vector2;
use tokio::io::{AsyncWriteExt};
use std::io;

use crate::network::utils;

#[derive(Debug)]
pub struct EntityUpdateData {
    pub id: u32,
    pub pos: Vector2<f64>
}

impl EntityUpdateData {
    pub const ID: u8 = 2;

    pub async fn write(&self, buf: &mut Vec<u8>) -> io::Result<()> {
        buf.write_u8(EntityUpdateData::ID).await?;
        buf.write_u32_le(self.id).await?;
        utils::write_pos(buf, self.pos).await?;
        Ok(())
    }
}