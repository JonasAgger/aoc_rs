use std::collections::HashMap;

use super::OpCodeArgument;


pub struct Memory {
    instructions: Vec<i64>,
    relative_base: i64, 
    extended_memory: HashMap<usize, i64>
}

impl Memory {
    pub fn new(instructions: Vec<i64>) -> Self {
        Self { 
            instructions, 
            relative_base: 0,
            extended_memory: HashMap::new()
        }
    }

    pub fn get_value(&self, address: usize) -> i64 {
        if address >= self.instructions.len() {
            match self.extended_memory.get(&address) {
                Some(val) => return *val,
                None => return 0,
            }
        }
        self.instructions[address]
    }

    pub fn set_value(&mut self, address: usize, value: i64) {
        if address >= self.instructions.len() {
            self.extended_memory.insert(address, value);
        }
        else {
            self.instructions[address] = value;
        }
    }

    pub fn get(&self, argument: OpCodeArgument) -> i64 {
        let address = match argument {
            OpCodeArgument::Position(pos) => pos as usize,
            OpCodeArgument::Immediate(val) => return val,
            OpCodeArgument::Relative(val) => (self.relative_base + val) as usize,
        };

        self.get_value(address)
    }

    pub fn set(&mut self, argument: OpCodeArgument, value: i64) {
        let address = match argument {
            OpCodeArgument::Position(pos) => pos as usize,
            OpCodeArgument::Relative(val) => (self.relative_base + val) as usize,
            OpCodeArgument::Immediate(_) => panic!("trying to set an immediate value"),
        };

        self.set_value(address, value);
    }

    pub fn add_relative_base(&mut self, value: i64) {
        self.relative_base += value;
    }

    pub fn instructions(&self) -> &[i64] {
        &self.instructions
    }
}