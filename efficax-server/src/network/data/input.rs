use tokio::io::{self, AsyncReadExt};

use std::io::Cursor;

#[derive(Debug)]
pub struct InputData {
    pub input: u8
}

impl InputData {
    pub const ID: u8 = 0;

    pub async fn parse(reader: &mut Cursor<&Vec<u8>>) -> io::Result<Self> {
        let input = reader.read_u8().await?;
        Ok(InputData {
            input
        })
    }
}