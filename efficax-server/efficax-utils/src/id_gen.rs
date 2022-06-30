pub struct IdGen {
    next_id: u32
}

impl IdGen {
    pub fn new(first_id: u32) -> Self {
        IdGen {
            next_id: first_id
        }
    }

    pub fn get(&mut self) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}