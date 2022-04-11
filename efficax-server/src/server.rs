use tokio::sync::mpsc::error::TryRecvError;
use tokio::sync::mpsc::{UnboundedReceiver};
use tokio::task::{self, JoinHandle};

use crate::network::message::NetworkMessage;
use crate::state::{EfficaxState};

pub async fn start(mut message_channel: UnboundedReceiver<NetworkMessage>) -> JoinHandle<()> {
    task::spawn_blocking(move || {
        let mut state = EfficaxState::new();
        loop {
            loop {
                match message_channel.try_recv() {
                    Ok(_message) => {
                        
                    }
                    Err(TryRecvError::Empty) => {
                        break;
                    }
                    Err(TryRecvError::Disconnected) => break
                }
            }
            state.tick();
        }
    })
}