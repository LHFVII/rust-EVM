use std::collections::HashMap;

pub struct Storage<'a, 'b> {
    db: HashMap<&'a str, u8>,
    cache: Vec<&'b str>,
}

impl<'a, 'b> Storage<'a, 'b> {
    pub fn new() -> Self {
        Storage {
            db: HashMap::new(),
            cache: Vec::new(),
        }
    }
    pub fn base_load(self, key: &str) -> u8 {
        return self.db[key];
    }
    pub fn load(mut self, key: &'b str) -> (bool, u8) {
        let mut warm = true;
        if !self.cache.contains(&key) {
            warm = false;
            self.cache.push(key);
        }
        if !self.db.contains_key(key) {
            return (warm, 0x00);
        }
        return (warm, self.db[key]);
    }
    pub fn store(mut self, key: &'a str, value: u8) {
        self.db.insert(key, value);
    }
}
