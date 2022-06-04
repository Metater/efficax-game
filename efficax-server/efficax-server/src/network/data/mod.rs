use self::types::PositionData;

pub mod types;
pub mod impls;

/* efficax-game schema

S->C:
    TCP packet: (3 + n bytes)
        data_len: u16
        tick_id: u8
        NetworkData...

    UDP packet: (1 + n bytes)
        tick_id: u8
        NetworkData...
C->S:
    TCP packet: (2 + n bytes)
        data_len: u16
        NetworkData...

    UDP packet: (n bytes)
        NetworkData...

NetworkData: (1 + n bytes)
    variant: u8
    0 Input: (UDP C->S) (2 bytes)
        input: u8
        input_sequence: u8
    1 Chat: (TCP S->C | TCP C->S) (? bytes)
        message: String
    2 Snapshot: (UDP S->C) (1 + n bytes)
        entity_snapshots_len: u8
        entity_snapshots: EntitySnapshotData...
        EntitySnapshotData: (6 + n bytes)
            id: u64
            pos: PositionData
            EntitySpecificSnapshotData...
            EntitySpecificSnapshotData: (1 + n bytes)
                variant: u8
                0 Player: (1 byte)
                    input_sequence: u8
    3 InitUDP: (TCP C->S) (2 bytes)
        udp_port: u16
    4 Join: (TCP S->C)
    5 Leave: (TCP S->C)

Shared:
    PositionData: (4 bytes)
        x: u16
        y: u16
*/

#[derive(Debug)]
pub enum NetworkData {
    Input(InputData),
    Chat(ChatData),
    Snapshot(SnapshotData),
    InitUDP(u16),
    Join(JoinData),
    Leave(LeaveData),
}

// Input
#[derive(bincode::Decode, Debug)]
pub struct InputData {
    pub input: u8,
    pub input_sequence: u8,
}

// Chat
#[derive(bincode::Encode, bincode::Decode, Debug)]
pub struct ChatData {
    pub message: String
}

// Snapshot
#[derive(Debug)]
pub struct SnapshotData {
    pub entity_snapshots: Vec<EntitySnapshotData>,
}

#[derive(bincode::Encode, Debug)]
pub struct EntitySnapshotData {
    pub id: u64,
    pub pos: PositionData,
    pub data: EntitySpecificSnapshotData,
}

#[derive(Debug)]
pub enum EntitySpecificSnapshotData {
    // input_sequence
    Player(PlayerSnapshotData)
}

#[derive(bincode::Encode, Debug)]
pub struct PlayerSnapshotData {
    pub input_sequence: u8
}

// Join
#[derive(bincode::Encode, Debug)]
pub struct JoinData {

}

// Leave
#[derive(bincode::Encode, Debug)]
pub struct LeaveData {

}