use tokio::io::{self, AsyncReadExt};

use std::io::Cursor;

#[derive(Debug)]
pub struct ChatData {
    pub message: String
}

impl ChatData {
    pub const ID: u8 = 1;

    pub async fn read(reader: &mut Cursor<&Vec<u8>>) -> io::Result<Self> {
        let mut message = String::new();
        reader.read_to_string(&mut message).await?;
        Ok(ChatData {
            message
        })
    }
}