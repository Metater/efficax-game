// file mods
pub mod message;
pub mod packet;

// dir mods
pub mod data;

// private file mods
mod listener;
mod sender;

// private dir mods

use tokio::sync::mpsc::{self,UnboundedReceiver, UnboundedSender};

use self::{sender::NetworkSender, listener::NetworkListener, message::{NetworkListenerMessage, NetworkSenderMessage}};

pub struct EfficaxNetwork {
    listener: NetworkListener,
    sender: NetworkSender
}

impl EfficaxNetwork {
    pub async fn start() -> (Self, UnboundedReceiver<NetworkListenerMessage>, UnboundedSender<NetworkSenderMessage>) {
        let (listener_tx, listener_rx) = mpsc::unbounded_channel::<NetworkListenerMessage>();
        let (mut sender_tx, sender_rx) = mpsc::unbounded_channel::<NetworkSenderMessage>();

        let listener = NetworkListener::start(listener_tx, &mut sender_tx).await;
        let sender = NetworkSender::start(sender_rx).await;
        
        let network = EfficaxNetwork {
            listener,
            sender
        };

        (network, listener_rx, sender_tx)
    }

    pub fn stop(&self) {
        self.listener.stop();
        self.sender.stop();
    }
}