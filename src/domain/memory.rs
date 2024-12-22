pub struct Memory {
    storage: Vec<Vec<u8>>,
}

impl Memory{
    pub fn new()-> Self{
        Memory{
            storage: vec![]
        }
    }
    pub fn access(&self,offset: usize, size: usize)-> &[u8]{
        let slice = &self.storage[offset][0..size];
        return slice;
    }
    pub fn load(&self, offset: usize)-> &[u8]{
        self.access(offset, 32)
    }
    pub fn store(&mut self, offset: usize, mut value: Vec<u8>){
        let mut current_length = value.len();
        while current_length < 32 {
            value.push(0);
            current_length = current_length + 1;
        }
        if offset+1 <= self.storage.len(){
            self.storage[offset] = value;
            return;
        }
        else if offset == self.storage.len(){
            self.storage.push(value);
            return;
        }
        eprintln!("memory overflow");
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store_and_load() {
        let mut memory = Memory::new();
        memory.store(0, vec![0x02]);
        assert_eq!(memory.load(0)[0],2);
        assert_eq!(memory.load(0).len(),32);
    }

    
}