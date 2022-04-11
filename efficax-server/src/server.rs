use tokio::sync::mpsc::error::TryRecvError;
use tokio::sync::mpsc::{self, UnboundedSender, UnboundedReceiver};
use tokio::task::{self, JoinHandle};

use crate::network::message::NetworkMessage;

pub async fn start(mut message_channel: UnboundedReceiver<NetworkMessage>) -> JoinHandle<()> {
    task::spawn_blocking(move || {
        main_loop(message_channel);
    })
}

pub fn main_loop(mut message_channel: UnboundedReceiver<NetworkMessage>) {
    loop {
        match message_channel.try_recv() {
            Ok(message) => {

            }
            Err(TryRecvError::Empty) => {
                
            }
            Err(TryRecvError::Disconnected) => break
        }
    }
}