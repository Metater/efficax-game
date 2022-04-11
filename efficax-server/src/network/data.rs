pub mod input;
pub mod chat;

use self::{input::InputData, chat::ChatData};

#[derive(Debug)]
pub enum NetworkData {
    Input(InputData),
    Chat(ChatData)
}