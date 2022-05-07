use self::types::PositionData;

pub mod types;
pub mod impls;

#[derive(Debug)]
pub enum NetworkData {
    Input(InputData),
    Chat(ChatData),
    TickUpdate(TickUpdateData)
}

// Input
#[derive(bincode::Encode, bincode::Decode, Debug)]
pub struct InputData {
    pub input: u8,
    pub input_sequence: u8,
}

// Chat
#[derive(bincode::Encode, bincode::Decode, Debug)]
pub struct ChatData {
    pub message: String
}

// TickUpdate
#[derive(Debug)]
pub struct TickUpdateData {
    pub entity_updates: Vec<EntityUpdateData>,
}

#[derive(bincode::Encode, bincode::Decode, Debug)]
pub struct EntityUpdateData {
    pub id: u64,
    pub pos: PositionData,
    pub input_sequence: u8,
}