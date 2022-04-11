use std::net::SocketAddr;

use std::time::Duration;
use std::thread::sleep;
use tokio::sync::mpsc::error::TryRecvError;
use tokio::sync::mpsc::{UnboundedReceiver};
use tokio::task::{self, JoinHandle};

use crate::network::client::NetworkClient;
use crate::network::data::NetworkData;
use crate::network::data::chat::ChatData;
use crate::network::message::NetworkMessage;
use crate::network::packet::NetworkPacket;
use crate::state::{EfficaxState};

pub async fn start(mut message_channel: UnboundedReceiver<NetworkMessage>) {
    task::spawn_blocking(move || {
        let mut server = EfficaxServer::new();
        'main_loop: loop {
            'recv_loop: loop {
                match message_channel.try_recv() {
                    Ok(message) => {
                        server.handle_message(message);
                    }
                    Err(TryRecvError::Empty) => {
                        //println!("message channel empty");
                        break 'recv_loop
                    }
                    Err(TryRecvError::Disconnected) => {
                        println!("message channel disconnected");
                        break 'main_loop
                    }
                }
            }
            server.tick();
            sleep(Duration::from_millis(20));
        }
        server.stop();
    }).await.unwrap()
}

struct EfficaxServer {
    clients: Vec<NetworkClient>,
    state: EfficaxState
}

impl EfficaxServer {
    pub fn new() -> Self {
        EfficaxServer {
            clients: Vec::new(),
            state: EfficaxState::new()
        }
    }

    pub fn stop(&mut self) {

    }

    pub fn tick(&mut self) {
        self.state.tick();
    }

    pub fn handle_message(&mut self, message: NetworkMessage) {
        match message {
            NetworkMessage::Join(client) => self.handle_join(client),
            NetworkMessage::Data(packet) => self.handle_data(packet),
            NetworkMessage::Leave(addr) => self.handle_leave(addr),
        }
    }
    
    fn handle_join(&mut self, client: NetworkClient) {
        println!("client {} joined server", client.addr);
        self.clients.push(client);
    }

    fn handle_data(&mut self, packet: NetworkPacket) {
        match packet.data {
            NetworkData::Input(ref data) => {

            }
            NetworkData::Chat(ref data) => {
                println!("client {} sent chat message: {}", packet.from, data.message);
            }
        }
        println!("client {} sent packet: {:?}", packet.from, packet.data);
    }

    fn handle_leave(&mut self, addr: SocketAddr) {
        println!("client {} left server", addr);
        let mut contains_client = false;
        let mut index = 0;
        for (i, client) in self.clients.iter().enumerate() {
            if client.addr == addr {
                contains_client = true;
                index = i;
            }
        }
        if contains_client {
            self.clients.remove(index);
        }
    }
}