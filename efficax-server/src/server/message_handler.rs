use std::net::SocketAddr;

use crate::{network::{message::NetworkListenerMessage, packet::NetworkPacket, data::NetworkData}};

use super::EfficaxServer;

impl EfficaxServer {
    pub fn handle_message(&mut self, message: NetworkListenerMessage) {
        match message {
            NetworkListenerMessage::Join(addr) => self.handle_join(addr),
            NetworkListenerMessage::Data(packet) => self.handle_data(packet),
            NetworkListenerMessage::Leave(addr) => self.handle_leave(addr),
        }
    }
    
    fn handle_join(&mut self, addr: SocketAddr) {
        println!("[server]: client: {} joined server", addr);
        self.state.new_client(addr);
    }
    
    fn handle_data(&mut self, packet: NetworkPacket) {
        match packet.data {
            NetworkData::Input(ref data) => {
                if let Some(player) = self.state.get_client(&packet.addrs[0]) {
                    player.feed_input(data);
                }
                //println!("client {} sent input data: {}", packet.from, data.input);
            }
            NetworkData::Chat(ref _data) => {
                //println!("client {} sent chat data: {}", packet.from, data.message);
            }
            _ => ()
        }
        println!("[server]: client: {} sent packet: {:?}", packet.addrs[0], packet.data);
    }
    
    fn handle_leave(&mut self, addr: SocketAddr) {
        println!("[server]: client: {} left server", addr);
        if let Some(client) = self.state.clients.remove(&addr) {
            self.state.zone.despawn_entity(client.id);
        }
    }
}