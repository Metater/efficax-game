pub struct IdGen {
    next_id: u64
}

impl IdGen {
    pub fn new(first_id: u64) {
        IdGen {
            next_id: first_id
        }
    }

    pub fn get(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}