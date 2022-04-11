mod server;
mod network;
mod state;

//use tokio::time::{sleep, Duration};
use tokio::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    println!("Hello, world!");

    /*
    stream.write_all(b"Hello, world!").await?;
    println!("sent data to: {}", &addr);
    */

    let (listen_task, message_channel) = network::start().await;
    let _server_task = server::start(message_channel).await;
    
    listen_task.abort();
    Ok(())
}