use anyhow::Result;

use crate::{utils::*, vm::VM};

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
        tx.send(2).unwrap();
        vm.execute();

        let mut result = 0;
        while let Ok(val) = rx.try_recv() {
            result = val;
        }

        Ok(result.into())       }
}