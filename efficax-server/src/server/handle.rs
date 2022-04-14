use std::sync::{atomic::{AtomicBool, Ordering}, Arc};

pub struct ServerHandle {
    should_stop: Arc<AtomicBool>,
}

impl ServerHandle {
    pub fn new() -> Self {
        ServerHandle {
            should_stop: Arc::new(AtomicBool::new(false))
        }
    }

    pub fn get_should_stop(&self) -> Arc<AtomicBool> {
        self.should_stop.clone()
    }
    
    pub fn stop(&self) {
        self.should_stop.store(true, Ordering::Relaxed);
    }
}