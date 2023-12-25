use anyhow::Result;

use crate::utils::*;
use crate::vm::VM;
use super::super::AocDay;


pub struct Day {

}

impl Day {
    pub fn new() -> Self {
        Self {}
    }
}

impl AocDay for Day {
    fn run_part1(&mut self, input: &[String]) -> Result<AoCResult> {
        let instruction_set: Vec<i64> =
            input[0].split(',').filter_map(|p| p.parse().ok()).collect();

        let mut vm = VM::new(instruction_set);
        let (tx, rx) = vm.use_channels();
        tx.send(1).unwrap();
        vm.execute();

        let mut result = 0;
        while let Ok(val) = rx.try_recv() {
            result = val;
        }

        Ok(result.into())    
    }

    fn run_part2(&mut self, input: &[String]) -> Result<AoCResult> {
        let instruction_set: Vec<i64> =
            input[0].split(',').filter_map(|p| p.parse().ok()).collect();

        let mut vm = VM::new(instruction_set);
        let (tx, rx) = vm.use_channels();
        tx.send(5).unwrap();
        vm.execute();

        let val = rx.recv().unwrap();
        Ok(val.into())        
    }
}

#[cfg(test)]
mod test {

    use std::mem;

    use crate::vm::{OpCode, OpCodeArgument, memory::Memory};

    use super::*;

    #[test]
    fn testcase1() {
        let input: Vec<i64> = vec![1102,45,16,225];
        let memory = Memory::new(input);
        let instruction = OpCode::get_instruction(&memory, 0);

        assert_eq!(instruction, OpCode::Multiplication(
            OpCodeArgument::Immediate(45), 
            OpCodeArgument::Immediate(16), 
            OpCodeArgument::Position(225)
        ))
    }
}