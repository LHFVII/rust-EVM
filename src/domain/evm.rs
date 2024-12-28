use super::{memory::Memory, stack::Stack, storage::Storage};

pub struct EVM<'a, 'b, 'c> {
    pc: &'c usize,
    stack: Stack<u8>,
    memory: Memory,
    storage: Storage<'a, 'b>,
    program: Vec<u8>,
    gas: u32,
    calldata: Vec<u8>,
    value: u32,
    stop_flag: bool,
    revert_flag: bool,
    return_data: Vec<u8>,
    return_logs: Vec<u8>,
}

impl<'a, 'b, 'c> EVM<'a, 'b, 'c> {
    pub fn new(program: Vec<u8>, gas: u32, value: u32, calldata: Vec<u8>) -> Self {
        EVM {
            pc: &0,
            stack: Stack::new(),
            memory: Memory::new(),
            storage: Storage::new(),
            program: program,
            gas: gas,
            calldata: calldata,
            value: value,
            stop_flag: false,
            revert_flag: false,
            return_data: Vec::new(),
            return_logs: Vec::new(),
        }
    }
    pub fn peek(self) {
        self.program[*self.pc];
    }

    pub fn gas_dec(mut self, amount: u32) {
        if self.gas - amount < 0 {
            eprintln!("Out of gas");
            return;
        }
        self.gas = self.gas - amount;
    }

    pub fn can_execute_next_op_code(&self) -> bool {
        if *self.pc > self.program.len() - 1 {
            return false;
        }
        if self.stop_flag {
            return false;
        }
        if self.revert_flag {
            return false;
        }
        return true;
    }

    pub fn run(&mut self) {
        println!("Running...");
        while self.can_execute_next_op_code() {
            let op = self.program[*self.pc];
            println!("Operand is: {:?}:", op);
        }
    }

    pub fn reset(&mut self) {
        self.pc = &0;
        self.stack = Stack::new();
        self.memory = Memory::new();
        self.storage = Storage::new();
    }

    pub fn add_op_code(&mut self, opcode: u8) {
        self.program.push(opcode);
    }

    pub fn set_gas_for_instruction(&mut self, gas: u32) {
        self.gas = gas;
    }
}
