use self::types::*;

pub mod types;
pub mod impls;
pub mod constants;

// If you want to free up enums, route by tcp/udp, client/server

/* efficax-game schema

S->C:
    TCP packet: (6 + n bytes)
        data_len: u16
        tick_id: u32
        NetworkData...

    UDP packet: (4 + n bytes)
        tick_id: u32
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
        EntitySnapshotData: (8 + n bytes)
            id: u32
            pos: PositionData
            EntitySpecificSnapshotData...
            EntitySpecificSnapshotData: (1 + n bytes)
                variant: u8
                0 Player: (1 byte)
                    input_sequence: u8
    3 InitUDP: (TCP C->S) (2 bytes)
        udp_port: u16
    4 Join: (TCP S->C) (8 bytes)
        player_id: u32
        pos: PositionData

Shared:
    PositionData: (4 bytes)
        x: u16
        y: u16
    EntityTypeData: (1 byte)
        variant: u8
*/

#[allow(dead_code)]
#[derive(Debug)]
pub enum NetworkData {
    Input(InputData),
    Chat(ChatData),
    Snapshot(SnapshotData),
    InitUDP(u16),
    Join(JoinData),
    Spawn(SpawnData),
    Despawn(DespawnData),
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
    pub id: u32,
    pub pos: PositionData,
    pub data: EntitySpecificSnapshotData,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum EntitySpecificSnapshotData {
    None,
    Player(PlayerSnapshotData)
}

#[derive(bincode::Encode, Debug)]
pub struct PlayerSnapshotData {
    pub input_sequence: u8
}

// Join
#[derive(bincode::Encode, Debug)]
pub struct JoinData {
    pub player_id: u32,
    pub pos: PositionData
}

// Spawn
#[derive(bincode::Encode, Debug)]
pub struct SpawnData {
    pub entity_type: EntityTypeData,
    pub entity_id: u32,
    pub pos: PositionData
}

// Despawn
#[derive(bincode::Encode, Debug)]
pub struct DespawnData {
    pub entity_id: u32
}