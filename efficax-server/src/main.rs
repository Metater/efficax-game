mod server;
mod network;

use tokio::time::{sleep, Duration};
use tokio::task::{self, JoinHandle};

use std::{io, net::SocketAddr};

#[tokio::main]
async fn main() -> io::Result<()> {
    println!("Hello, world!");

    /*
    stream.write_all(b"Hello, world!").await?;
    println!("sent data to: {}", &addr);
    */

    // TODO Use poll receive for reading channel
    let (listen_task, message_channel) = network::start().await;

    
    listen_task.abort();
    Ok(())
}