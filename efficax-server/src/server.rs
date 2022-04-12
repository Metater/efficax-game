use std::net::SocketAddr;
use std::time::{Duration, Instant};
use std::thread::{sleep};
use std::collections::HashMap;

use tokio::sync::mpsc::error::TryRecvError;
use tokio::sync::mpsc::{UnboundedReceiver};
use tokio::task::{self};

use crate::network::client::NetworkClient;
use crate::network::data::NetworkData;
use crate::network::message::NetworkMessage;
use crate::network::packet::NetworkPacket;
use crate::state::{EfficaxState};

pub async fn run(mut rx: UnboundedReceiver<NetworkMessage>) {
    task::spawn_blocking(move || {
        let start_time = Instant::now();
        let mut server = EfficaxServer::new();
        'main_loop: loop {
            server.start_tick();
            'recv_loop: loop {
                match rx.try_recv() {
                    Ok(message) => {
                        server.handle_message(message);
                    }
                    Err(TryRecvError::Empty) => {
                        //println!("message channel empty");
                        break 'recv_loop
                    }
                    Err(TryRecvError::Disconnected) => {
                        println!("message sending channel disconnected");
                        break 'main_loop
                    }
                }
            }
            server.tick();
            server.end_tick();
        }
        println!("server stopped after {:?} and on tick: {}", start_time.elapsed(), server.state.tick_id);
        server.stop();
    }).await.unwrap()
}

struct EfficaxServer {
    tick_start: Instant,
    carryover_time: Duration,
    tick_period: Duration,

    clients: HashMap<SocketAddr, NetworkClient>,
    state: EfficaxState
}

impl EfficaxServer {
    pub fn new() -> Self {
        EfficaxServer {
            tick_start: Instant::now(),
            carryover_time: Duration::default(),
            tick_period: Duration::from_millis(40),

            clients: HashMap::new(),
            state: EfficaxState::new()
        }
    }

    pub fn stop(&mut self) {

    }

    pub fn start_tick(&mut self) {
        self.tick_start = Instant::now();
    }

    pub fn end_tick(&mut self) {
        while self.tick_start.elapsed() + self.carryover_time < self.tick_period {
            sleep(Duration::from_millis(1));
        }
        self.carryover_time = (self.tick_start.elapsed() + self.carryover_time) - self.tick_period;
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
        self.clients.insert(client.addr, client);
    }

    fn handle_data(&mut self, packet: NetworkPacket) {
        match packet.data {
            NetworkData::Input(ref _data) => {
                //println!("client {} sent input data: {}", packet.from, data.input);
            }
            NetworkData::Chat(ref _data) => {
                //println!("client {} sent chat data: {}", packet.from, data.message);
            }
        }
        println!("client {} sent packet: {:?}", packet.from, packet.data);
    }

    fn handle_leave(&mut self, addr: SocketAddr) {
        println!("client {} left server", addr);
        self.clients.remove(&addr);
    }
}