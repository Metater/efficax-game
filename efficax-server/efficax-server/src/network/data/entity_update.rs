use cgmath::Vector2;
use tokio::io::{AsyncWriteExt};
use std::io;

use efficax_utils::data;

use super::loadables::Vector2f32Data;

#[derive(bincode::Encode, bincode::Decode, Debug)]
pub struct EntityUpdateData {
    pub id: u64,
    pub pos: Vector2f32Data,
    pub input_sequence: u8,
}

impl EntityUpdateData {
    pub async fn write(&self, buf: &mut Vec<u8>) -> io::Result<()> {
        buf.write_u64_le(self.id).await?;
        data::write_pos(buf, self.pos).await?;
        buf.write_u8(self.input_sequence).await?;
        Ok(())
    }
}