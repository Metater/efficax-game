use super::{NetworkData, InputData, ChatData, SnapshotData, EntitySpecificSnapshotData};

// NetworkData
impl bincode::Encode for NetworkData {
    fn encode<E: bincode::enc::Encoder>(&self, encoder: &mut E) -> Result<(), bincode::error::EncodeError> {
        match self {
            NetworkData::Chat(data) => {
                <u8 as bincode::Encode>::encode(&1u8, encoder)?;
                data.encode(encoder)?;
            }
            NetworkData::Snapshot(data) => {
                <u8 as bincode::Encode>::encode(&2u8, encoder)?;
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
            0 => Ok(NetworkData::Input(<InputData as bincode::Decode>::decode(decoder)?)),
            1 => Ok(NetworkData::Chat(<ChatData as bincode::Decode>::decode(decoder)?)),
            3 => Ok(NetworkData::InitUDP(<u16 as bincode::Decode>::decode(decoder)?)),
            variant => Err(bincode::error::DecodeError::UnexpectedVariant {
                found: variant as u32,
                type_name: "NetworkData",
                allowed: bincode::error::AllowedEnumVariants::Allowed(&[0, 1, 3]),
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