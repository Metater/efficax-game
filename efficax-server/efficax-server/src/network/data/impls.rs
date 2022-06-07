use super::{NetworkData, InputData, ChatData, SnapshotData, EntitySpecificSnapshotData, INPUT, CHAT, INIT_UDP, SNAPSHOT, JOIN, SPAWN, DESPAWN};

// NetworkData
impl bincode::Encode for NetworkData {
    fn encode<E: bincode::enc::Encoder>(&self, encoder: &mut E) -> Result<(), bincode::error::EncodeError> {
        match self {
            NetworkData::Chat(data) => {
                <u8 as bincode::Encode>::encode(&CHAT, encoder)?;
                data.encode(encoder)?;
            }
            NetworkData::Snapshot(data) => {
                <u8 as bincode::Encode>::encode(&SNAPSHOT, encoder)?;
                data.encode(encoder)?;
            }
            NetworkData::Join(data) => {
                <u8 as bincode::Encode>::encode(&JOIN, encoder)?;
                data.encode(encoder)?;
            }
            NetworkData::Spawn(data) => {
                <u8 as bincode::Encode>::encode(&SPAWN, encoder)?;
                data.encode(encoder)?;
            }
            NetworkData::Despawn(data) => {
                <u8 as bincode::Encode>::encode(&DESPAWN, encoder)?;
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
            INPUT => Ok(NetworkData::Input(<InputData as bincode::Decode>::decode(decoder)?)),
            CHAT => Ok(NetworkData::Chat(<ChatData as bincode::Decode>::decode(decoder)?)),
            INIT_UDP => Ok(NetworkData::InitUDP(<u16 as bincode::Decode>::decode(decoder)?)),
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
            EntitySpecificSnapshotData::Player(data) => {
                <u8 as bincode::Encode>::encode(&0u8, encoder)?;
                data.encode(encoder)?;
            }
        }
        Ok(())
    }
}