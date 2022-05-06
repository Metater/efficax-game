pub mod input;
pub mod chat;
pub mod entity_update;
pub mod tick_update;
pub mod loadables;

use self::{input::InputData, chat::ChatData, tick_update::TickUpdateData};

#[derive(bincode::Encode, bincode::Decode, Debug)]
pub enum NetworkData {
    Input(InputData),
    Chat(ChatData),
    TickUpdate(TickUpdateData)
}