pub mod utils;

pub mod input;
pub mod chat;
pub mod entity_update;

use self::{input::InputData, chat::ChatData, entity_update::EntityUpdateData};

#[derive(Debug)]
pub enum NetworkData {
    Input(InputData),
    Chat(ChatData),
    EntityUpdate(EntityUpdateData)
}