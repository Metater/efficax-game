// file mods
pub mod handle;
pub mod message_handler;

// dir mods
pub mod metaitus;
pub mod state;
pub mod world;

// private file mods

// private dir mods

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::{sleep};

use tokio::sync::mpsc::error::TryRecvError;
use tokio::task::{self, JoinHandle};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::time::{Instant, Duration};

use crate::network::data::NetworkData;
use crate::network::data::entity_update::EntityUpdateData;
use crate::network::message::{NetworkListenerMessage, NetworkSenderMessage};
use crate::network::packet::NetworkPacket;

use self::handle::ServerHandle;
use self::state::ServerState;

pub async fn start(listener_rx: UnboundedReceiver<NetworkListenerMessage>, sender_tx: UnboundedSender<NetworkSenderMessage>) -> (ServerHandle, JoinHandle<()>) {
    let handle =  ServerHandle::new();
    let mut server = EfficaxServer::new(&handle, listener_rx, sender_tx);

    let server_task = task::spawn_blocking(move || {
        server.run();
    });

    (handle, server_task)
}

pub struct EfficaxServer {
    should_stop: Arc<AtomicBool>,

    listener_rx: UnboundedReceiver<NetworkListenerMessage>,
    pub sender_tx: UnboundedSender<NetworkSenderMessage>,

    pub state: ServerState,

    start_time: Instant,
    tick_start_time: Instant,
    carryover_time: Duration,
    tick_period: Duration,
}

impl EfficaxServer {
    pub fn new(handle: &ServerHandle, listener_rx: UnboundedReceiver<NetworkListenerMessage>, sender_tx: UnboundedSender<NetworkSenderMessage>) -> Self {
        EfficaxServer {
            should_stop: handle.get_should_stop(),

            listener_rx,
            sender_tx,

            state: ServerState::new(),

            start_time: Instant::now(),
            tick_start_time: Instant::now(),
            carryover_time: Duration::default(),
            tick_period: Duration::from_millis(40),
        }
    }

    pub fn run(&mut self) {
        self.main_loop();
        println!("[server]: stopped after: {:?} and on tick: {}", self.start_time.elapsed(), self.state.tick_id);
    }

    fn main_loop(&mut self) {
        while !self.should_stop.load(Ordering::Relaxed) {
            self.tick_start_time = Instant::now();

            // Break on listener channel disconnect
            if self.recv_loop() {
                break
            }
            self.tick();

            // Sleep until tick perioid has elapsed
            while self.tick_start_time.elapsed() + self.carryover_time < self.tick_period {
                sleep(Duration::from_millis(1));
            }
            // Carry over for sleep overrun
            self.carryover_time = (self.tick_start_time.elapsed() + self.carryover_time) - self.tick_period;
        }
    }

    fn recv_loop(&mut self) -> bool {
        loop {
            match self.listener_rx.try_recv() {
                Ok(message) => {
                    message_handler::handle_message(self, message);
                }
                Err(TryRecvError::Empty) => {
                    break
                }
                Err(TryRecvError::Disconnected) => {
                    println!("[server]: listener channel disconnected");
                    return true;
                }
            }
        }
        return false;
    }

    fn tick(&mut self) {
        self.state.tick();

        self.client_movement();
    }

    fn client_movement(&mut self) {
        let addrs = self.state.get_addrs();
        let clients = self.state.get_clients_mut();

        for player in clients {
            player.apply_input();
            for &addr in &addrs {
                self.sender_tx.send(NetworkSenderMessage::Data(
                    NetworkPacket::new(addr, NetworkData::EntityUpdate(EntityUpdateData {
                        id: player.id,
                        pos: player.pos,
                        rotation: player.last_input
                    }))
                )).ok();
            }  
        }
    }
}