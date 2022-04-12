pub mod message;
pub mod packet;
pub mod data;

use tokio::{net::{TcpListener, tcp::{OwnedReadHalf, OwnedWriteHalf}}, task::JoinHandle};
use tokio::sync::mpsc::{self, UnboundedSender, UnboundedReceiver};

use std::{io::{self, Cursor}, net::SocketAddr, collections::HashMap};

use crate::network::message::{NetworkListenerMessage, NetworkSenderMessage};
use crate::network::packet::NetworkPacket;

pub async fn open() -> (EfficaxNetwork, UnboundedReceiver<NetworkListenerMessage>, UnboundedSender<NetworkSenderMessage>) {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    let (listener_tx, listener_rx) = mpsc::unbounded_channel::<NetworkListenerMessage>();
    let (sender_tx, sender_rx) = mpsc::unbounded_channel::<NetworkSenderMessage>();

    let listener_sender_tx = sender_tx.clone();

    let listener = tokio::spawn(async move {
        EfficaxNetwork::listen(listener, listener_tx, listener_sender_tx).await;
        println!("[network listener]: stopped");
    });

    let sender = tokio::spawn(async move {
        EfficaxNetwork::send(sender_rx).await;
        println!("[network sender]: stopped");
    });

    // sender

    (EfficaxNetwork::new(listener, sender), listener_rx, sender_tx)
}

pub struct EfficaxNetwork {
    listener: JoinHandle<()>,
    sender: JoinHandle<()>
}

impl EfficaxNetwork {
    pub fn new(listener: JoinHandle<()>, sender: JoinHandle<()>) -> Self {
        EfficaxNetwork {
            listener,
            sender
        }
    }

    pub fn stop(&self) {
        self.listener.abort();
        self.sender.abort();
    }

    async fn listen(listener: TcpListener, listener_tx: UnboundedSender<NetworkListenerMessage>, sender_tx: UnboundedSender<NetworkSenderMessage>) {
        println!("[network listener]: started");
        loop {
            let (stream, addr) = match listener.accept().await {
                Ok(stream) => stream,
                Err(_) => continue
            };
    
            let (reader, writer) = stream.into_split();
    
            let listener_channel = listener_tx.clone();
            if let Err(_) = listener_channel.send(NetworkListenerMessage::Join(addr)) {
                break;
            }

            let sender_channel = sender_tx.clone();
            if let Err(_) = sender_channel.send(NetworkSenderMessage::Join((addr, writer))) {
                break;
            }
    
            //println!("accepted client: {}", addr);
    
            tokio::spawn(async move {
                if let Err(e) = EfficaxNetwork::process(&listener_channel, reader, addr).await {
                    println!("[network listener]: client: {} error: {}", addr, e);
                }
                listener_channel.send(NetworkListenerMessage::Leave(addr)).ok();
                sender_channel.send(NetworkSenderMessage::Leave(addr)).ok();
            });
        }
    }
    
    async fn process(
        channel: &UnboundedSender<NetworkListenerMessage>,
        reader: OwnedReadHalf,
        addr: SocketAddr
    ) -> io::Result<()> {
        let mut buf = Vec::with_capacity(4096);
        loop {
            reader.readable().await?;
            match reader.try_read_buf(&mut buf) {
                Ok(0) => break,
                Ok(n) => {
                    println!("[network listener]: client: {} sent: {} bytes", addr, n);
                    let mut reader = Cursor::new(&buf);
                    // if reader.position is 4096, carry over data?
                    while reader.position() < n as u64 {
                        let packet = NetworkPacket::read(addr, &mut reader).await?;
                        let message = NetworkListenerMessage::Data(packet);
                        if let Err(_) = channel.send(message) {
                            break;
                        }
                    }
                    //println!("client {} sent data {:#?}", addr, buf);
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(e) => {
                    return Err(e.into());
                }
            }
            buf.clear();
        }
    
        Ok(())
    }

    async fn send(mut rx: UnboundedReceiver<NetworkSenderMessage>) {
        println!("[network sender]: started");
        let mut clients: HashMap<SocketAddr, OwnedWriteHalf> = HashMap::new();
        while let Some(message) = rx.recv().await {
            match message {
                NetworkSenderMessage::Join((addr, writer)) => {
                    clients.insert(addr, writer);
                }
                NetworkSenderMessage::Data(packet) => {
                    if let Some(writer) = clients.get(&packet.addr) {
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