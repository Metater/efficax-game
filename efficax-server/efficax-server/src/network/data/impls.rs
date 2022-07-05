use super::{NetworkData, InputData, ChatData, SnapshotData, EntitySpecificSnapshotData, types::EntityTypeData};

// NetworkData
impl bincode::Encode for NetworkData {
    fn encode<E: bincode::enc::Encoder>(&self, encoder: &mut E) -> Result<(), bincode::error::EncodeError> {
        match self {
            NetworkData::Chat(data) => {
                <u8 as bincode::Encode>::encode(&NetworkData::CHAT, encoder)?;
                data.encode(encoder)?;
            }
            NetworkData::Snapshot(data) => {
                <u8 as bincode::Encode>::encode(&NetworkData::SNAPSHOT, encoder)?;
                data.encode(encoder)?;
            }
            NetworkData::Join(data) => {
                <u8 as bincode::Encode>::encode(&NetworkData::JOIN, encoder)?;
                data.encode(encoder)?;
            }
            NetworkData::Spawn(data) => {
                <u8 as bincode::Encode>::encode(&NetworkData::SPAWN, encoder)?;
                data.encode(encoder)?;
            }
            NetworkData::Despawn(data) => {
                <u8 as bincode::Encode>::encode(&NetworkData::DESPAWN, encoder)?;
                data.encode(encoder)?;
            }
            data => {
                panic!("encoding: {:?} not supported", data);
            }
        }
        Ok(())
    }
}
impl bincode::Decode for NetworkData {
    fn decode<D: bincode::de::Decoder>(decoder: &mut D) -> Result<Self, bincode::error::DecodeError> {
        let variant_index = <u8 as bincode::Decode>::decode(decoder)?;
        match variant_index {
            NetworkData::INPUT => Ok(NetworkData::Input(<InputData as bincode::Decode>::decode(decoder)?)),
            NetworkData::CHAT => Ok(NetworkData::Chat(<ChatData as bincode::Decode>::decode(decoder)?)),
            NetworkData::INIT_UDP => Ok(NetworkData::InitUDP(<u16 as bincode::Decode>::decode(decoder)?)),
            variant => Err(bincode::error::DecodeError::UnexpectedVariant {
                found: variant as u32,
                type_name: "NetworkData",
                allowed: bincode::error::AllowedEnumVariants::Allowed(&[]),
            })
        }
    }
}

// SnapshotData
impl bincode::Encode for SnapshotData {
    fn encode<E: bincode::enc::Encoder>(&self, encoder: &mut E) -> Result<(), bincode::error::EncodeError> {
        <u8 as bincode::Encode>::encode(&(self.entity_snapshots.len() as u8), encoder)?;
        for entity_snapshot in &self.entity_snapshots {
            entity_snapshot.encode(encoder)?;
        }
        Ok(())
    }
}

// EntitySpecificSnapshotData
impl bincode::Encode for EntitySpecificSnapshotData {
    fn encode<E: bincode::enc::Encoder>(&self, encoder: &mut E) -> Result<(), bincode::error::EncodeError> {
        match self {
            EntitySpecificSnapshotData::None => {
                <u8 as bincode::Encode>::encode(&EntitySpecificSnapshotData::NONE, encoder)?;
            }
            EntitySpecificSnapshotData::Player(data) => {
                <u8 as bincode::Encode>::encode(&EntitySpecificSnapshotData::PLAYER, encoder)?;
                data.encode(encoder)?;
            }
        }
        Ok(())
    }
}

// EntityTypeData
impl bincode::Encode for EntityTypeData {
    fn encode<E: bincode::enc::Encoder>(&self, encoder: &mut E) -> Result<(), bincode::error::EncodeError> {
        match self {
            EntityTypeData::Player => {
                <u8 as bincode::Encode>::encode(&EntityTypeData::PLAYER, encoder)?;
            }
        }
        Ok(())
    }
}