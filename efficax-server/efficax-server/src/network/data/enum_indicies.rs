use super::{types::EntityType, EntitySnapshotDataType, traits::EnumAsU8};

impl EnumAsU8 for EntitySnapshotDataType {
    fn as_u8(&self) -> u8 {
        match self {
            EntitySnapshotDataType::Position(_) => 0,
            EntitySnapshotDataType::Player(_) => 0,
        }
    }
}

impl EnumAsU8 for EntityType {
    fn as_u8(&self) -> u8 {
        match self {
            EntityType::Player => 0,
        }
    }
}