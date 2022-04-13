use std::collections::HashMap;
use std::net::SocketAddr;

use tokio::net::tcp::OwnedWriteHalf;
use tokio::{task::JoinHandle, sync::mpsc::UnboundedReceiver};

use super::message::NetworkSenderMessage;

pub struct NetworkSender {
    sender: JoinHandle<()>
}

impl NetworkSender {
    pub async fn start(sender_rx: UnboundedReceiver<NetworkSenderMessage>) -> Self {
        let sender = tokio::spawn(async move {
            NetworkSender::send(sender_rx).await;
            println!("[network sender]: stopped");
        });

        NetworkSender {
            sender
        }
    }

    pub fn stop(&self) {
        self.sender.abort();
    }

    async fn send(mut sender_rx: UnboundedReceiver<NetworkSenderMessage>) {
        println!("[network sender]: started");
        let mut clients: HashMap<SocketAddr, OwnedWriteHalf> = HashMap::new();
        while let Some(message) = sender_rx.recv().await {
            match message {
                NetworkSenderMessage::Join((addr, writer)) => {
                    clients.insert(addr, writer);
                }
                NetworkSenderMessage::Data(packet) => {
                    if let Some(writer) = clients.get_mut(&packet.addr) {
                        packet.send(writer).await;
                    }
                    else {
                        println!("[network sender]: tried to send data: {:?} to missing client: {}", packet.data, packet.addr);
                    }
                }
                NetworkSenderMessage::Leave(addr) => {
                    clients.remove(&addr);
                }
            };
        }
    }
}