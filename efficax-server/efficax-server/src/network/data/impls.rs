use super::{NetworkData, InputData, ChatData, TickUpdateData, EntityUpdateData};

// NetworkData
impl bincode::Encode for NetworkData {
    fn encode<E: bincode::enc::Encoder>(&self, encoder: &mut E) -> Result<(), bincode::error::EncodeError> {
        match self {
            NetworkData::Input(data) => {
                <u8 as bincode::Encode>::encode(&0u8, encoder)?;
                data.encode(encoder)?;
            }
            NetworkData::Chat(data) => {
                <u8 as bincode::Encode>::encode(&1u8, encoder)?;
                data.encode(encoder)?;
            }
            NetworkData::TickUpdate(data) => {
                <u8 as bincode::Encode>::encode(&2u8, encoder)?;
                data.encode(encoder)?;
            }
            NetworkData::SetUDPPort(data) => {
                <u8 as bincode::Encode>::encode(&3u8, encoder)?;
                data.encode(encoder)?;
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
            2 => Ok(NetworkData::TickUpdate(<TickUpdateData as bincode::Decode>::decode(decoder)?)),
            3 => Ok(NetworkData::SetUDPPort(<u16 as bincode::Decode>::decode(decoder)?)),
            variant => Err(bincode::error::DecodeError::UnexpectedVariant {
                found: variant as u32,
                type_name: "NetworkData",
                allowed: bincode::error::AllowedEnumVariants::Range { min: 0, max: 3 }
            })
        }
    }
}

// TickUpdateData
impl bincode::Encode for TickUpdateData {
    fn encode<E: bincode::enc::Encoder>(&self, encoder: &mut E) -> Result<(), bincode::error::EncodeError> {
        <u8 as bincode::Encode>::encode(&(self.entity_updates.len() as u8), encoder)?;
        for entity_update in &self.entity_updates {
            entity_update.encode(encoder)?;
        }
        Ok(())
    }
}
impl bincode::Decode for TickUpdateData {
    fn decode<D: bincode::de::Decoder>(decoder: &mut D) -> Result<Self, bincode::error::DecodeError> {
        let len = <u8 as bincode::Decode>::decode(decoder)?;
        let mut entity_updates= Vec::new();
        for _ in 0..len {
            entity_updates.push(<EntityUpdateData as bincode::Decode>::decode(decoder)?);
        }
        Ok(TickUpdateData {
            entity_updates
        })
    }
}