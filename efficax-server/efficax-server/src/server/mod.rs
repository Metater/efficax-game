pub mod state;
pub mod constants;

use std::collections::HashMap;
use std::net::SocketAddr;
use std::thread::sleep;
use std::sync::{atomic::{AtomicBool, Ordering}, Arc};

use tokio::sync::mpsc::error::TryRecvError;
use tokio::task::{self, JoinHandle};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::time::{Instant, Duration};

use crate::network::{NetworkReceiverMessage, NetworkSenderMessage};

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

    udp_to_tcp_addrs: HashMap<SocketAddr, SocketAddr>,
    tcp_to_udp_addrs: HashMap<SocketAddr, SocketAddr>,

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

            udp_to_tcp_addrs: HashMap::new(),
            tcp_to_udp_addrs: HashMap::new(),

            state: ServerState::new(sender_tx),

            start_time: Instant::now(),
            tick_start_time: Instant::now(),
            carryover_time: Duration::ZERO,
            tick_period: Duration::from_millis(40),
        }
    }

    pub fn run(&mut self) {
        self.init();
        self.main_loop();
        println!("[server]: stopped after: {:?} and on tick: {}", self.start_time.elapsed(), self.state.tick_id);
    }

    fn init(&mut self) {
        self.state.init();
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
            NetworkReceiverMessage::Join(addr) => {
                println!("[server]: client: {} joined server", addr);
                self.state.join(addr);
            }
            NetworkReceiverMessage::Leave(addr) => {
                if let Some(udp_addr) = self.tcp_to_udp_addrs.remove(&addr) {
                    self.udp_to_tcp_addrs.remove(&udp_addr);
                }
                println!("[server]: client: {} left server", addr);
                self.state.leave(addr);
            }
            NetworkReceiverMessage::InitNetwork((addr, udp_port)) => {
                let udp_addr = SocketAddr::new(addr.ip(), udp_port);
                self.udp_to_tcp_addrs.insert(udp_addr, addr);
                self.tcp_to_udp_addrs.insert(addr, udp_addr);
            }
            NetworkReceiverMessage::Data(packet) => {
                let mut addr = packet.get_addr();
                if !packet.is_tcp {
                    if let Some(&tcp_addr) = self.udp_to_tcp_addrs.get(&addr) {
                        addr = tcp_addr;
                    }
                    else {
                        return;
                    }
                }
                self.state.data(packet, addr);
            }
        }
    }
}