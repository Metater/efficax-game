pub mod message;
pub mod packet;
pub mod data;
pub mod client;

use tokio::{net::{TcpListener, tcp::OwnedReadHalf}, task::JoinHandle};
use tokio::sync::mpsc::{self, UnboundedSender, UnboundedReceiver};

use std::{io::{self, Cursor}, net::SocketAddr};

use crate::network::{message::NetworkMessage, client::NetworkClient};
use crate::network::packet::NetworkPacket;

pub async fn start() -> (JoinHandle<()>, UnboundedReceiver<NetworkMessage>) {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    let (tx, rx) = mpsc::unbounded_channel::<NetworkMessage>();

    let listen_task = tokio::spawn(async move {
        listen(listener, tx).await;
    });

    (listen_task, rx)
}

async fn listen(listener: TcpListener, tx: UnboundedSender<NetworkMessage>) {
    println!("started listening");
    loop {
        // TODO May need accept on listener.accept for cancel to work correctly
        let (stream, addr) = match listener.accept().await {
            Ok(stream) => stream,
            Err(_) => continue
        };

        let (reader, writer) = stream.into_split();

        let channel = tx.clone();

        let client = NetworkClient::new(addr, writer);
        if let Err(_) = channel.send(NetworkMessage::Join(client)) {
            break;
        }

        //println!("accepted client: {}", addr);

        tokio::spawn(async move {
            if let Err(e) = process(&channel, reader, addr).await {
                println!("client {} error: {}", addr, e);
            }
            channel.send(NetworkMessage::Leave(addr)).ok();
        });
    }
}

async fn process(
    channel: &UnboundedSender<NetworkMessage>,
    reader: OwnedReadHalf,
    addr: SocketAddr
) -> io::Result<()> {
    let mut buf = Vec::with_capacity(4096);
    loop {
        reader.readable().await?;
        match reader.try_read_buf(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                println!("client {} sent {} bytes", addr, n);
                let mut reader = Cursor::new(&buf);
                    // if reader.position is 4096, carry over data?
                while reader.position() < n as u64 {
                    let packet = NetworkPacket::parse(addr, &mut reader).await?;
                    let message = NetworkMessage::Data(packet);
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