pub mod player_state;

pub struct EfficaxState {
    pub tick_id: u64
}

impl EfficaxState {
    pub fn new() -> EfficaxState {
        EfficaxState {
            tick_id: 0,
        }
    }

    pub fn tick(&mut self) {
        
        self.tick_id += 1;
    }
}