//#![allow(dead_code)]

mod server;
mod network;

use std::io::{stdin, stdout, Read, Write};

#[tokio::main]
async fn main() {
    println!("[server]: Hello, world!!");

    let (receiver_rx, sender_tx, receiver_stop_notifier, receiver_handle, sender_handle) = network::start().await;
    let sender_stop_notifier = sender_tx.clone();
    let (server, server_task) = server::start(receiver_rx, sender_tx);

    ctrlc::set_handler(move || {
        if server.is_running() {
            println!("[server]: stopping");
            receiver_stop_notifier.notify_waiters();
            sender_stop_notifier.send(network::NetworkSenderMessage::Stop);
            server.stop();
        }
    }).expect("Error setting Ctrl-C handler");

    receiver_handle.await.unwrap();
    sender_handle.await.unwrap();
    server_task.await.unwrap();

    println!("[server]: stopped");

    println!("[server]: Press Enter to continue...");
    stdout().flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}