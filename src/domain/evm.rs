use std::collections::HashMap;

use super::{memory::Memory, stack::Stack, storage::Storage};

pub struct EVM<'a, 'b> {
    pc: usize,
    stack: Stack<u8>,
    memory: Memory,
    storage: Storage<'a, 'b>,
    program: Vec<u8>,
    gas: i32,
    stop_flag: bool,
    revert_flag: bool,
    gas_cost: HashMap<u8, i32>,
    calldata: Vec<u8>,
    value: u32,
    return_data: Vec<u8>,
    return_logs: Vec<u8>,
}

impl<'a, 'b> EVM<'a, 'b> {
    pub fn new(program: Vec<u8>, gas: i32, value: u32, calldata: Vec<u8>) -> Self {
        EVM {
            pc: 0,
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
            gas_cost: HashMap::from([(1, 3), (2, 4), (3, 5)]),
        }
    }
    pub fn peek(&mut self) -> u8 {
        return self.program[self.pc];
    }

    pub fn gas_dec(&mut self, amount: i32) {
        if (self.gas - amount) < 0 {
            eprintln!("Out of gas");
            return;
        }
        self.gas = self.gas - amount;
    }

    pub fn can_execute_next_op_code(&self) -> bool {
        if self.pc > self.program.len() - 1 {
            return false;
        }
        if self.stop_flag {
            return false;
        }
        if self.revert_flag {
            return false;
        }
        if self.gas < 1 {
            return false;
        }
        return true;
    }

    pub fn run(&mut self) {
        println!("Running...");
        /*if !self.is_gas_enough() {
            eprintln!("Not enough gas");
            return;
        } */
        while self.can_execute_next_op_code() {
            let op = self.program[self.pc];
            self.execute_opcode(op);
        }
    }

    pub fn reset(&mut self) {
        self.pc = 0;
        self.stack = Stack::new();
        self.memory = Memory::new();
        self.storage = Storage::new();
    }

    pub fn add_op_code(&mut self, opcode: u8) {
        self.program.push(opcode);
    }

    pub fn set_gas_for_instruction(&mut self, gas: i32) {
        self.gas = gas;
    }

    pub fn is_gas_enough(&mut self) -> bool {
        let mut expected_cost = 0;
        for i in &self.program {
            let current_cost = self.gas_cost.get(i).unwrap();
            expected_cost += *current_cost;
        }
        return self.gas >= expected_cost;
    }

    pub fn println_stack(&self) {
        println!("Printing stack...");
        for i in self.stack.iter() {
            println!("{:?}", *i);
        }
    }

    fn execute_opcode(&mut self, opcode: u8) {
        if opcode == 1 {
            return self.add();
        }
        if opcode == 96 {
            return self.push_one();
        }
    }

    fn add(&mut self) {
        let a = self.stack.pop().unwrap().unwrap();
        let b = self.stack.pop().unwrap().unwrap();
        let _ = self.stack.push(a + b);
        self.pc += 1;
        self.gas_dec(3);
    }

    fn push_one(&mut self) {
        self.pc += 1;
        self.gas_dec(3);
        let value = self.peek();
        self.pc += 1;
        let _ = self.stack.push(value);
    }
}
