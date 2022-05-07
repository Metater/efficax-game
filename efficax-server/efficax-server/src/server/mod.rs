pub mod state;
pub mod world;

use std::net::SocketAddr;
use std::thread::{sleep};
use std::sync::{atomic::{AtomicBool, Ordering}, Arc};

use tokio::sync::mpsc::error::TryRecvError;
use tokio::task::{self, JoinHandle};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::time::{Instant, Duration};

use crate::network::{NetworkReceiverMessage, NetworkSenderMessage};
use crate::network::data::NetworkData;
use crate::network::packet::NetworkPacket;

use self::state::ServerState;

pub struct ServerHandle {
    running: Arc<AtomicBool>,
}

impl ServerHandle {
    pub fn new() -> Self {
        ServerHandle {
            running: Arc::new(AtomicBool::new(true))
        }
    }

    pub fn get_new_handle(&self) -> ServerHandle {
        ServerHandle {
            running: self.running.clone()
        }
    }
    
    pub fn stop(&self) {
        self.running.store(false, Ordering::Relaxed);
    }

    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::Relaxed)
    }
}

pub fn start(receiver_rx: UnboundedReceiver<NetworkReceiverMessage>, sender_tx: UnboundedSender<NetworkSenderMessage>) -> (ServerHandle, JoinHandle<()>) {
    let handle =  ServerHandle::new();
    let mut server = EfficaxServer::new(&handle, receiver_rx, sender_tx);

    let server_task = task::spawn_blocking(move || {
        server.run();
    });

    (handle, server_task)
}

pub struct EfficaxServer {
    handle: ServerHandle,

    receiver_rx: UnboundedReceiver<NetworkReceiverMessage>,

    state: ServerState,

    start_time: Instant,
    tick_start_time: Instant,
    carryover_time: Duration,
    tick_period: Duration,
}

impl EfficaxServer {
    pub fn new(handle: &ServerHandle, receiver_rx: UnboundedReceiver<NetworkReceiverMessage>, sender_tx: UnboundedSender<NetworkSenderMessage>) -> Self {
        EfficaxServer {
            handle: handle.get_new_handle(),

            receiver_rx,

            state: ServerState::new(sender_tx),

            start_time: Instant::now(),
            tick_start_time: Instant::now(),
            carryover_time: Duration::ZERO,
            tick_period: Duration::from_millis(40),
        }
    }

    pub fn run(&mut self) {
        self.main_loop();
        println!("[server]: stopped after: {:?} and on tick: {}", self.start_time.elapsed(), self.state.tick_id);
    }

    fn main_loop(&mut self) {
        while self.handle.is_running() {
            self.tick_start_time = Instant::now();

            // break on receiver channel disconnect
            if self.recv_loop() {
                break
            }

            self.tick();

            // sleep until tick period has elapsed
            while self.get_delta_time() < self.tick_period {
                sleep(Duration::from_millis(1));
            }

            // carryover for sleep overrun
            self.carryover_time = self.get_delta_time() - self.tick_period;
        }
    }

    fn recv_loop(&mut self) -> bool {
        loop {
            match self.receiver_rx.try_recv() {
                Ok(message) => {
                    self.handle_message(message);
                }
                Err(TryRecvError::Empty) => {
                    return false;
                }
                Err(TryRecvError::Disconnected) => {
                    println!("[server]: receiver channel disconnected");
                    return true;
                }
            }
        }
    }

    fn tick(&mut self) {
        self.state.tick(self.tick_period.as_secs_f32());
    }

    fn get_delta_time(&self) -> Duration {
        self.tick_start_time.elapsed() + self.carryover_time
    }
}

impl EfficaxServer {
    pub fn handle_message(&mut self, message: NetworkReceiverMessage) {
        match message {
            NetworkReceiverMessage::Join(addr) => self.handle_join(addr),
            NetworkReceiverMessage::Leave(addr) => self.handle_leave(addr),
            NetworkReceiverMessage::Data(packet) => self.handle_data(packet),
        }
    }
    
    fn handle_join(&mut self, addr: SocketAddr) {
        println!("[server]: client: {} joined server", addr);
        self.state.join(addr);
    }

    fn handle_leave(&mut self, addr: SocketAddr) {
        println!("[server]: client: {} left server", addr);
        self.state.leave(addr);
    }
    
    fn handle_data(&mut self, packet: NetworkPacket) {
        match packet.data {
            NetworkData::Input(ref data) => {
                self.state.input_data(packet.get_addr(), data);
            }
            _ => {
                println!("[server]: client: {} sent unhandleable packet: {:?}", packet.get_addr(), packet.data);
            }
        }
    }
}