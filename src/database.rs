use std::collections::HashMap;

// todo: mutex should be used to avoid mutual exclusion

use crate::data::Data;

pub struct Db {
    state: HashMap<String, Data>,
    // n_entries: u32,
}

impl Db {
    pub fn new() -> Self {
        Db {
            state: HashMap::new(),
            // n_entries: 0,
        }
    }

    // add custom error & return Result
    pub fn set(&mut self, key: &str, value: Data) -> Option<Data> {
        self.state.insert(key.to_string(), value)
    }

    pub fn get(&self, key: &str) -> Option<&Data> {
        self.state.get(key)
    }
}
