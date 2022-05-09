use self::types::PositionData;

pub mod types;
pub mod impls;

#[derive(Debug)]
pub enum NetworkData {
    Input(InputData),
    Chat(ChatData),
    TickUpdate(TickUpdateData),
    InitUDP(u16),
    //Join(JoinData),
    //Leave(LeaveData),
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
    pub tick_id: u8,
    pub entity_updates: Vec<EntityUpdateData>,
}

#[derive(bincode::Encode, bincode::Decode, Debug)]
pub struct EntityUpdateData {
    pub id: u64,
    pub pos: PositionData,
    pub input_sequence: u8,
}

/*
#[derive(bincode::Encode, bincode::Decode, Debug)]
pub struct JoinData {

}

#[derive(bincode::Encode, bincode::Decode, Debug)]
pub struct LeaveData {

}
 */