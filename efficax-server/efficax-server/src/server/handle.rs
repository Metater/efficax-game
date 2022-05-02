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

    pub fn get_handle(&self) -> ServerHandle {
        ServerHandle {
            should_stop: self.should_stop.clone()
        }
    }
    
    pub fn stop(&self) {
        self.should_stop.store(true, Ordering::Relaxed);
    }

    pub fn is_running(&self) -> bool {
        !self.should_stop.load(Ordering::Relaxed)
    }
}