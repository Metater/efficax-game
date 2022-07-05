pub mod world;
pub mod net;

pub mod client_state;
pub mod physics;
pub mod utils;

use std::{net::SocketAddr, collections::HashMap};

use cgmath::Vector2;
use tokio::sync::mpsc::UnboundedSender;

use metaitus::{zone::MetaitusZone, collider::MetaitusCollider};

use crate::network::{NetworkSenderHandle, NetworkSenderMessage};

use self::client_state::ClientState;

pub struct ServerState {
    pub tick_id: u32,

    pub clients: HashMap<SocketAddr, ClientState>,

    zone: MetaitusZone,

    net: NetworkSenderHandle,
}

impl ServerState {
    pub fn new(sender_tx: UnboundedSender<NetworkSenderMessage>) -> Self {
        ServerState {
            tick_id: 0,

            clients: HashMap::new(),

            zone: MetaitusZone::new(),

            net: NetworkSenderHandle::new(sender_tx)
        }
    }

    pub fn init(&mut self) {
        //self.zone.add_static(MetaitusCollider::new(Vector2::new(2.0, 0.0), Vector2::new(3.0, 1.0)));
        //self.zone.add_static(MetaitusCollider::new(Vector2::new(5.0, 0.0), Vector2::new(6.0, 1.0)));

        self.zone.add_static(MetaitusCollider::new(Vector2::new(0.25, 0.25), Vector2::new(0.75, 0.75)));
        self.zone.add_static(MetaitusCollider::new(Vector2::new(0.75, 0.75), Vector2::new(1.25, 1.25)));
    }

    pub fn tick(&mut self, delta_time: f32) {
        self.tick_physics(delta_time);

        self.tick_net_out();

        self.tick_id += 1;
    }
}