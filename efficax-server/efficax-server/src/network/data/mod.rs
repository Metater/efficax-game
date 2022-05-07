pub mod loadables;

use self::loadables::Vector2f32Data;

#[derive(bincode::Encode, bincode::Decode, Debug)]
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
#[derive(bincode::Encode, bincode::Decode, Debug)]
pub struct TickUpdateData {
    pub entity_updates: Vec<EntityUpdateData>,
}

#[derive(bincode::Encode, bincode::Decode, Debug)]
pub struct EntityUpdateData {
    pub id: u64,
    pub pos: Vector2f32Data,
    pub input_sequence: u8,
}