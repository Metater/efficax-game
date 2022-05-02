// file mods
pub mod handle;
pub mod message_handler;

// dir mods
pub mod state;
pub mod world;

// private file mods

// private dir mods

use std::thread::{sleep};

use tokio::sync::mpsc::error::TryRecvError;
use tokio::task::{self, JoinHandle};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::time::{Instant, Duration};

use crate::network::message::{NetworkListenerMessage, NetworkSenderMessage};

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
    handle: ServerHandle,

    listener_rx: UnboundedReceiver<NetworkListenerMessage>,
    sender_tx: UnboundedSender<NetworkSenderMessage>,

    state: ServerState,

    start_time: Instant,
    tick_start_time: Instant,
    carryover_time: Duration,
    tick_period: Duration,
}

impl EfficaxServer {
    pub fn new(handle: &ServerHandle, listener_rx: UnboundedReceiver<NetworkListenerMessage>, sender_tx: UnboundedSender<NetworkSenderMessage>) -> Self {
        EfficaxServer {
            handle: handle.get_handle(),

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
        while self.handle.is_running() {
            self.tick_start_time = Instant::now();

            // break on listener channel disconnect
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
            match self.listener_rx.try_recv() {
                Ok(message) => {
                    self.handle_message(message);
                }
                Err(TryRecvError::Empty) => {
                    return false;
                }
                Err(TryRecvError::Disconnected) => {
                    println!("[server]: listener channel disconnected");
                    return true;
                }
            }
        }
    }

    fn tick(&mut self) {
        //let delta_time = self.get_delta_time().as_secs_f32();
        let sender_tx = &mut self.sender_tx;
        self.state.tick(self.tick_period.as_secs_f32(), sender_tx);
    }

    fn get_delta_time(&self) -> Duration {
        self.tick_start_time.elapsed() + self.carryover_time
    }
}