use tokio::io::{AsyncWriteExt};
use std::io;

use super::entity_update::EntityUpdateData;

#[derive(Debug)]
pub struct TickUpdateData {
    pub entity_updates: Vec<EntityUpdateData>,
}

impl TickUpdateData {
    pub const ID: u8 = 3;

    pub async fn write(&self, buf: &mut Vec<u8>) -> io::Result<()> {
        buf.write_u8(TickUpdateData::ID).await?;
        buf.write_u8(self.entity_updates.len() as u8).await?;
        for entity_update in &self.entity_updates {
            entity_update.write(buf).await?;
        }
        Ok(())
    }
}