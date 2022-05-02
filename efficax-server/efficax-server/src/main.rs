//#![allow(dead_code)]

mod server;
mod network;

use std::io::{stdin, stdout, Read, Write};

use crate::network::EfficaxNetwork;

#[tokio::main]
async fn main() {
    println!("[server]: Hello, world!!");

    let (network, listener_rx, sender_tx) = EfficaxNetwork::start().await;
    let (server, server_task) = server::start(listener_rx, sender_tx);

    ctrlc::set_handler(move || {
        if server.is_running() {
            println!("[server]: stopping");
            network.stop();
            server.stop();
        }
    }).expect("Error setting Ctrl-C handler");

    server_task.await.unwrap();

    println!("[server]: stopped");

    println!("[server]: Press Enter to continue...");
    stdout().flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}