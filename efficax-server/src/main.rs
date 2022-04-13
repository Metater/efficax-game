mod server;
mod network;

// Next packet index = 3

use crate::network::EfficaxNetwork;

#[tokio::main]
async fn main() {
    println!("[server]: Hello, world!");

    let (network, listener_rx, sender_tx) = EfficaxNetwork::start().await;

    ctrlc::set_handler(move || {
        println!("[server]: stopping");
        network.stop();
    }).expect("Error setting Ctrl-C handler");

    server::run(listener_rx, sender_tx).await;

    // allow server.stop to be called eventually by ctrl c
    // find out what delays network.stop

    println!("[server]: stopped");
    //sleep(Duration::from_secs(5)).await;
}