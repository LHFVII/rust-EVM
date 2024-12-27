pub struct Memory {
    cells: Vec<Vec<u8>>,
}

impl Memory {
    pub fn new() -> Self {
        Memory { cells: vec![] }
    }
    pub fn access(&self, offset: usize, size: usize) -> &[u8] {
        let slice = &self.cells[offset][0..size];
        return slice;
    }
    pub fn load(&self, offset: usize) -> &[u8] {
        self.access(offset, 32)
    }
    pub fn store(&mut self, offset: usize, mut value: Vec<u8>) {
        let mut current_length = value.len();
        while current_length < 32 {
            value.push(0);
            current_length = current_length + 1;
        }
        if offset + 1 <= self.cells.len() {
            self.cells[offset] = value;
            return;
        } else if offset == self.cells.len() {
            self.cells.push(value);
            return;
        }
        eprintln!("memory overflow");
    }
    pub fn calc_memory_expansion_gas(memory_byte_size: usize) -> usize {
        let memory_size_word = (memory_byte_size + 31) / 32;
        let memory_cost = (memory_size_word.pow(2)) / 512 + (3 * memory_size_word);
        return memory_cost;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store_and_load() {
        let mut memory = Memory::new();
        memory.store(0, vec![0x02]);
        assert_eq!(memory.load(0)[0], 2);
        assert_eq!(memory.load(0).len(), 32);
    }
}
