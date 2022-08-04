use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

pub struct Waitlist {
    list: Arc<Mutex<HashSet<i64>>>,
}

impl Waitlist {
    pub fn new() -> Self {
        Waitlist {
            list: Arc::new(Mutex::new(HashSet::new())),
        }
    }

    pub fn add_to_waitlist(&mut self, id: i64) -> bool {
        self.list.lock().unwrap().insert(id)
    }

    pub fn remove_from_waitlist(&mut self, id: i64) -> bool {
        self.list.lock().unwrap().remove(&id)
    }
}

impl Clone for Waitlist {
    fn clone(&self) -> Waitlist {
        Waitlist {
            list: Arc::clone(&self.list),
        }
    }
}
