use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

use tokio::net::tcp::OwnedWriteHalf;
use tokio::sync::Notify;
use tokio::{task::JoinHandle, sync::mpsc::UnboundedReceiver};

use super::NetworkSenderMessage;

pub async fn start(sender_rx: UnboundedReceiver<NetworkSenderMessage>) -> JoinHandle<()> {
    let stop_notifier = Arc::new(Notify::new());
    let sender_stop_notifier = stop_notifier.clone();

    let handle = tokio::spawn(async move {
        start_sending(sender_rx).await;
        println!("[network sender]: stopped");
    });

    handle
}

async fn start_sending(mut sender_rx: UnboundedReceiver<NetworkSenderMessage>) {
    println!("[network sender]: started");
    let mut clients: HashMap<SocketAddr, OwnedWriteHalf> = HashMap::new();
    while let Some(message) = sender_rx.recv().await {
        match message {
            NetworkSenderMessage::Join((addr, writer)) => {
                clients.insert(addr, writer);
            }
            NetworkSenderMessage::Leave(addr) => {
                clients.remove(&addr);
            }
            NetworkSenderMessage::Data(packet) => {
                packet.send(&mut clients).await;
            }
            NetworkSenderMessage::Stop => {
                break;
            }
        };
    }
}