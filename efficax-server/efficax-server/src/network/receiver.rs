use std::{net::{SocketAddr}, io::Cursor, collections::VecDeque};

use byteorder::{LittleEndian, ByteOrder};
use tokio::{io, task::JoinHandle, sync::mpsc::UnboundedSender, net::{TcpListener, tcp::OwnedReadHalf}};

use super::{packet::NetworkPacket, NetworkReceiverMessage, NetworkSenderMessage, data::NetworkData};

pub struct NetworkListener {
    listener: JoinHandle<()>
}

impl NetworkListener {
    const BUF_SIZE: usize = 4096;
    const RING_SIZE: usize = 8192;

    pub async fn start(listener_tx: UnboundedSender<NetworkReceiverMessage>, sender_tx: &mut UnboundedSender<NetworkSenderMessage>) -> Self {
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

    async fn listen(listener: TcpListener, listener_tx: UnboundedSender<NetworkReceiverMessage>, sender_tx: UnboundedSender<NetworkSenderMessage>) {
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

    async fn process(receiver_tx: &UnboundedSender<NetworkReceiverMessage>, reader: OwnedReadHalf, addr: SocketAddr) -> io::Result<()> {
        let mut buf = [0u8; Self::BUF_SIZE];
        let mut ring = VecDeque::with_capacity(Self::RING_SIZE);
        loop {
            reader.readable().await?;
            match reader.try_read(&mut buf) {
                Ok(0) => break,
                Ok(n) => {
                    println!("[network listener]: client: {} sent: {} bytes", addr, n);
                    ring.extend(&buf[..n]);
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(e) => {
                    return Err(e.into());
                }
            }
            /*
            let mut reader = Cursor::new(&buf);
            while reader.position() < n as u64 {
                let packet = NetworkPacket::read(addr, &mut reader).await?;
                let message = NetworkReceiverMessage::Data(packet);
                if let Err(_) = receiver_tx.send(message) {
                    break;
                }
            }
            */

            let available_data = ring.len();

            if available_data > Self::RING_SIZE {
                return Err(io::Error::new(io::ErrorKind::Other, "unexpectedly expanded ring buffer"));
            }

            if available_data > 2 {
                let slice = ring.make_contiguous();
                while available_data > 2 {
                    let declared_packet_size = LittleEndian::read_u16(&slice[..2]);
                    if declared_packet_size > 4096 {
                        return Err(io::Error::new(io::ErrorKind::Other, "declared packet size too large"));
                    }
                    if available_data < declared_packet_size as usize + 2 {
                        continue;
                    }
                    // 2 will need offset after while looping
                    let (data, len): (NetworkData, usize) = bincode::decode_from_slice(&slice[2..declared_packet_size], bincode::config::standard()).unwrap();

                    available_data = ring.len();
                }
            }
        }
    
        Ok(())
    }
}