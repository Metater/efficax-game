//#![allow(dead_code)]

// file mods
mod utils;

// dir mods
mod server;
mod network;

// private file mods

// private dir mods

// Next packet index = 3

use std::io::{stdin, stdout, Read, Write};

use crate::network::EfficaxNetwork;

#[tokio::main]
async fn main() {
    println!("[server]: Hello, world!");

    // only allow stop once

    // ensure all constructors have -> Self
    // std, other libs, efficax

    // add velocity epsilon

    let (network, listener_rx, sender_tx) = EfficaxNetwork::start().await;
    let (server, server_task) = server::start(listener_rx, sender_tx).await;

    ctrlc::set_handler(move || {
        println!("[server]: stopping");
        network.stop();
        server.stop();
    }).expect("Error setting Ctrl-C handler");

    server_task.await.unwrap();

    // allow server.stop to be called eventually by ctrl c
    // find out what delays network.stop

    println!("[server]: stopped");

    println!("[server]: Press Enter to continue...");
    stdout().flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}