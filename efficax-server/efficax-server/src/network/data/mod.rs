pub mod input;
pub mod chat;
pub mod entity_update;
pub mod tick_update;

use self::{input::InputData, chat::ChatData, tick_update::TickUpdateData};

#[derive(Debug)]
pub enum NetworkData {
    Input(InputData),
    Chat(ChatData),
    TickUpdate(TickUpdateData)
}