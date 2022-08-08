use super::{NetworkData, InputData, ChatData, SnapshotData, EntitySnapshotDataType, types::EntityType, traits::EnumAsU8};

// NetworkData
impl bincode::Encode for NetworkData {
    fn encode<E: bincode::enc::Encoder>(&self, encoder: &mut E) -> Result<(), bincode::error::EncodeError> {
        match self {
            NetworkData::Chat(data) => {
                <u8 as bincode::Encode>::encode(&self.as_u8(), encoder)?;
                data.encode(encoder)?;
            }
            NetworkData::Snapshot(data) => {
                <u8 as bincode::Encode>::encode(&self.as_u8(), encoder)?;
                data.encode(encoder)?;
            }
            NetworkData::Join(data) => {
                <u8 as bincode::Encode>::encode(&self.as_u8(), encoder)?;
                data.encode(encoder)?;
            }
            NetworkData::Spawn(data) => {
                <u8 as bincode::Encode>::encode(&self.as_u8(), encoder)?;
                data.encode(encoder)?;
            }
            NetworkData::Despawn(data) => {
                <u8 as bincode::Encode>::encode(&self.as_u8(), encoder)?;
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
            NetworkData::INIT_UDP => Ok(NetworkData::InitNetwork(<u16 as bincode::Decode>::decode(decoder)?)),
            variant => Err(bincode::error::DecodeError::UnexpectedVariant {
                found: variant as u32,
                type_name: "NetworkData",
                allowed: bincode::error::AllowedEnumVariants::Allowed(&[]),
            })
        }
    }
}

impl bincode::Encode for SnapshotData {
    fn encode<E: bincode::enc::Encoder>(&self, encoder: &mut E) -> Result<(), bincode::error::EncodeError> {
        <u8 as bincode::Encode>::encode(&(self.entity_snapshots.len() as u8), encoder)?;
        for entity_snapshot in &self.entity_snapshots {
            entity_snapshot.encode(encoder)?;
        }
        Ok(())
    }
}

impl bincode::Encode for EntitySnapshotDataType {
    fn encode<E: bincode::enc::Encoder>(&self, encoder: &mut E) -> Result<(), bincode::error::EncodeError> {
        match self {
            EntitySnapshotDataType::Position(data) => {
                <u8 as bincode::Encode>::encode(&self.as_u8(), encoder)?;
                data.encode(encoder)?;
            }
            EntitySnapshotDataType::Player(data) => {
                <u8 as bincode::Encode>::encode(&self.as_u8(), encoder)?;
                data.encode(encoder)?;
            }
        }
        Ok(())
    }
}

impl bincode::Encode for EntityType {
    fn encode<E: bincode::enc::Encoder>(&self, encoder: &mut E) -> Result<(), bincode::error::EncodeError> {
        <u8 as bincode::Encode>::encode(&self.as_u8(), encoder)?;
        Ok(())
    }
}