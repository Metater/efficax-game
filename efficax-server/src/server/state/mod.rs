pub mod player_state;

pub struct ServerState {
    pub tick_id: u64,

    next_entity_id: u32,
}

impl ServerState {
    pub fn new() -> ServerState {
        ServerState {
            tick_id: 0,

            next_entity_id: 0
        }
    }

    pub fn tick(&mut self) {
        self.tick_id += 1;
    }

    pub fn get_next_entity_id(&mut self) -> u32 {
        let entity_id = self.next_entity_id;
        self.next_entity_id += 1;
        entity_id
    }
}