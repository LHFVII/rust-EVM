use std::collections::HashMap;

pub struct Storage<T,V> {
    db: HashMap<T,V>
}

impl<T,V> Storage<T,V>{
    pub fn new()-> Self{
        Storage { db: HashMap::<T,V>::new() }
    }
    pub fn load (self, key: T)-> V{
        return self.db[key];
    }
    pub fn store(self, key:T, value: V){
        self.db.insert(key,value);
    } 
    
}