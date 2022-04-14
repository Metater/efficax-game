use std::net::SocketAddr;

use crate::{network::{message::NetworkListenerMessage, packet::NetworkPacket, data::NetworkData}};

use super::EfficaxServer;

pub fn handle_message(server: &mut EfficaxServer, message: NetworkListenerMessage) {
    match message {
        NetworkListenerMessage::Join(addr) => handle_join(server, addr),
        NetworkListenerMessage::Data(packet) => handle_data(server, packet),
        NetworkListenerMessage::Leave(addr) => handle_leave(server, addr),
    }
}

fn handle_join(server: &mut EfficaxServer, addr: SocketAddr) {
    println!("[server]: client: {} joined server", addr);
    server.state.new_client(addr);
}

fn handle_data(server: &mut EfficaxServer, packet: NetworkPacket) {
    match packet.data {
        NetworkData::Input(ref data) => {
            if let Some(player) = server.state.get_client(&packet.addr) {
                player.feed_input(data);
            }
            //println!("client {} sent input data: {}", packet.from, data.input);
        }
        NetworkData::Chat(ref _data) => {
            //println!("client {} sent chat data: {}", packet.from, data.message);
        }
        _ => ()
    }
    println!("[server]: client: {} sent packet: {:?}", packet.addr, packet.data);
}

fn handle_leave(server: &mut EfficaxServer, addr: SocketAddr) {
    println!("[server]: client: {} left server", addr);
    server.state.clients.remove(&addr);
}