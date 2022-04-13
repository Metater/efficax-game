use std::net::SocketAddr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};
use std::thread::{sleep};
use std::collections::{HashSet, HashMap};

use cgmath::{Point2, Vector2};
use tokio::sync::mpsc::error::TryRecvError;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::task::{self};

use crate::network::data::NetworkData;
use crate::network::data::entity_update::EntityUpdateData;
use crate::network::message::{NetworkListenerMessage, NetworkSenderMessage};
use crate::network::packet::NetworkPacket;
use crate::state::player_state::PlayerState;
use crate::state::{EfficaxState};

pub async fn run(mut rx: UnboundedReceiver<NetworkListenerMessage>, tx: UnboundedSender<NetworkSenderMessage>) {
    task::spawn_blocking(move || {
        let start_time = Instant::now();
        let mut server = EfficaxServer::new(tx);
        'main_loop: loop {
            if !*server.run.get_mut() {
                break 'main_loop;
            }
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
                        println!("[server]: listener channel disconnected");
                        break 'main_loop
                    }
                }
            }
            server.tick();
            server.end_tick();
        }
        println!("[server]: stopped after: {:?} and on tick: {}", start_time.elapsed(), server.state.tick_id);
        server.stop();
    }).await.unwrap()
}

struct EfficaxServer {
    run: AtomicBool,
    
    tick_start: Instant,
    carryover_time: Duration,
    tick_period: Duration,

    sender: UnboundedSender<NetworkSenderMessage>,
    //clients: HashSet<SocketAddr>,
    state: EfficaxState,

    players: HashMap<SocketAddr, PlayerState>,
}

impl EfficaxServer {
    pub fn new(sender: UnboundedSender<NetworkSenderMessage>) -> Self {
        EfficaxServer {
            run: AtomicBool::new(true),

            tick_start: Instant::now(),
            carryover_time: Duration::default(),
            tick_period: Duration::from_millis(40),

            sender,
            //clients: HashSet::new(),
            state: EfficaxState::new(),

            players: HashMap::new(),
        }
    }

    pub fn stop(&mut self) {
        self.run.store(false, Ordering::Relaxed);
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

        // send packets updates for each player to each player
        let addrs: Vec<SocketAddr> = self.players.keys().copied().collect();

        for player in &mut self.players {
            player.1.apply_input();
            for &addr in &addrs {
                self.sender.send(NetworkSenderMessage::Data(
                    NetworkPacket::new(addr, NetworkData::EntityUpdate(EntityUpdateData {
                        id: player.1.id,
                        pos: player.1.pos,
                    }))
                )).ok();
            }  
        }
    }

    pub fn handle_message(&mut self, message: NetworkListenerMessage) {
        match message {
            NetworkListenerMessage::Join(addr) => self.handle_join(addr),
            NetworkListenerMessage::Data(packet) => self.handle_data(packet),
            NetworkListenerMessage::Leave(addr) => self.handle_leave(addr),
        }
    }
    
    fn handle_join(&mut self, addr: SocketAddr) {
        println!("[server]: client: {} joined server", addr);
        self.players.insert(addr, PlayerState::new(self.state.get_next_entity_id(), Vector2::new(0.0, 0.0)));
    }

    fn handle_data(&mut self, packet: NetworkPacket) {
        match packet.data {
            NetworkData::Input(ref data) => {
                if let Some(player) = self.players.get_mut(&packet.addr) {
                    player.feed_input(data);
                }
                //println!("client {} sent input data: {}", packet.from, data.input);
            }
            NetworkData::Chat(ref _data) => {
                //println!("client {} sent chat data: {}", packet.from, data.message);
            }
            _ => {
                // TODO disconnect client
            }
        }
        println!("[server]: client: {} sent packet: {:?}", packet.addr, packet.data);
    }

    fn handle_leave(&mut self, addr: SocketAddr) {
        println!("[server]: client: {} left server", addr);
        self.players.remove(&addr);
    }
}