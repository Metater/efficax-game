use super::{NetworkData, EntitySpecificSnapshotData, types::EntityTypeData};

impl NetworkData {
    pub const INPUT: u8 = 0;
    pub const CHAT: u8 = 1;
    pub const SNAPSHOT: u8 = 2;
    pub const INIT_UDP: u8 = 3;
    pub const JOIN: u8 = 4;
    pub const SPAWN: u8 = 5;
    pub const DESPAWN: u8 = 6;
}

impl EntitySpecificSnapshotData {
    pub const NONE: u8 = 0;
    pub const PLAYER: u8 = 1;
}

impl EntityTypeData {
    pub const PLAYER: u8 = 0;
}