use tokio::time::{sleep, Duration};

mod server;
mod network;
mod state;
mod world;

// Next packet index = 3

#[tokio::main]
async fn main() {
    println!("[server]: Hello, world!");

    let (network, rx, tx) = network::open().await;

    ctrlc::set_handler(move || {
        println!("[server]: stopping");
        network.stop();
    }).expect("Error setting Ctrl-C handler");

    server::run(rx, tx).await;

    println!("[server]: stopped");
    sleep(Duration::from_secs(5)).await;
}