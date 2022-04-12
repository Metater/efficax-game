use tokio::time::{sleep, Duration};

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

    let (listener, rx) = network::listen().await;

    ctrlc::set_handler(move || {
        listener.stop();
        println!("server shutting down");
    }).expect("Error setting Ctrl-C handler");

    server::run(rx).await;

    sleep(Duration::from_secs(4)).await;
}