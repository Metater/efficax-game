//#![allow(dead_code)]

mod server;
mod network;

use std::io::{stdin, stdout, Read, Write};

#[tokio::main]
async fn main() {
    println!("[server]: Hello, world!!");

    let (receiver_rx,
        sender_tx,
        receiver_stop_notifier,
        receiver_handle,
        udp_receiver_handle,
        sender_handle) = network::start().await;
    let sender_stop_notifier = sender_tx.clone();
    let (server, server_task) = server::start(receiver_rx, sender_tx);

    ctrlc::set_handler(move || {
        receiver_stop_notifier.notify_waiters();
        if server.is_running() {
            println!("[server]: stopping");
            sender_stop_notifier.send(network::NetworkSenderMessage::Stop).ok();
            server.stop();
        }
    }).expect("Error setting Ctrl-C handler");

    receiver_handle.await.unwrap();
    udp_receiver_handle.await.unwrap();
    sender_handle.await.unwrap();
    
    server_task.await.unwrap();

    println!("[server]: stopped");

    println!("[server]: press enter to continue...");
    stdout().flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}