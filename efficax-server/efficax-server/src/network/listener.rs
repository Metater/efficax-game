use std::{net::{SocketAddr}, io::Cursor};

use tokio::{io, task::JoinHandle, sync::mpsc::UnboundedSender, net::{TcpListener, tcp::OwnedReadHalf}};

use super::{packet::NetworkPacket, NetworkListenerMessage, NetworkSenderMessage};

pub struct NetworkListener {
    listener: JoinHandle<()>
}

impl NetworkListener {
    pub async fn start(listener_tx: UnboundedSender<NetworkListenerMessage>, sender_tx: &mut UnboundedSender<NetworkSenderMessage>) -> Self {
        let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

        let sender_tx = sender_tx.clone();

        let listener = tokio::spawn(async move {
            NetworkListener::listen(listener, listener_tx, sender_tx).await;
            println!("[network listener]: stopped");
        });

        NetworkListener {
            listener
        }
    }

    pub fn stop(&self) {
        self.listener.abort();
    }

    async fn listen(listener: TcpListener, listener_tx: UnboundedSender<NetworkListenerMessage>, sender_tx: UnboundedSender<NetworkSenderMessage>) {
        println!("[network listener]: started");
        loop {
            let (stream, addr) = match listener.accept().await {
                Ok(client) => client,
                Err(_) => continue
            };

            if let Err(_) = stream.set_nodelay(true) {
                continue;
            }
    
            let (reader, writer) = stream.into_split();
    
            let listener_channel = listener_tx.clone();
            if let Err(_) = listener_channel.send(NetworkListenerMessage::Join(addr)) {
                break;
            }

            let sender_channel = sender_tx.clone();
            if let Err(_) = sender_channel.send(NetworkSenderMessage::Join((addr, writer))) {
                break;
            }
    
            tokio::spawn(async move {
                if let Err(e) = NetworkListener::process(&listener_channel, reader, addr).await {
                    println!("[network listener]: error: {} client: {}", e, addr);
                }
                listener_channel.send(NetworkListenerMessage::Leave(addr)).ok();
                sender_channel.send(NetworkSenderMessage::Leave(addr)).ok();
            });
        }
    }

    async fn process(channel: &UnboundedSender<NetworkListenerMessage>, reader: OwnedReadHalf, addr: SocketAddr) -> io::Result<()> {
        let mut buf = Vec::with_capacity(4096);
        loop {
            reader.readable().await?;
            match reader.try_read_buf(&mut buf) {
                Ok(0) => break,
                Ok(n) => {
                    println!("[network listener]: client: {} sent: {} bytes", addr, n);
                    let mut reader = Cursor::new(&buf);
                    while reader.position() < n as u64 {
                        let packet = NetworkPacket::read(addr, &mut reader).await?;
                        let message = NetworkListenerMessage::Data(packet);
                        if let Err(_) = channel.send(message) {
                            break;
                        }
                    }
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
}