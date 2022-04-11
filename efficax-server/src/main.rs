mod server;
mod network;
mod state;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    /*
    stream.write_all(b"Hello, world!").await?;
    println!("sent data to: {}", &addr);
    */

    let (listen_task, message_channel) = network::start().await;
    server::start(message_channel).await;
    
    println!("server shutting down");
    //listen_task.abort();
}