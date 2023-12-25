use anyhow::Result;
use itertools::Itertools;

use crate::{utils::*, vm::{VM, self}};

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

        let mut max = 0;
        let items: [i64; 5] = [0,1,2,3,4];
        
        for input in items.iter().permutations(items.len()).unique() {
            let mut vm1 = VM::new(instruction_set.clone());
            let mut vm2 = VM::new(instruction_set.clone());
            let mut vm3 = VM::new(instruction_set.clone());
            let mut vm4 = VM::new(instruction_set.clone());
            let mut vm5 = VM::new(instruction_set.clone());
    
            let (tx1, rx1) = vm1.use_channels();
            let (tx2, rx2) = vm2.use_channels();
            let (tx3, rx3) = vm3.use_channels();
            let (tx4, rx4) = vm4.use_channels();
            let (tx5, rx5) = vm5.use_channels();

            tx1.send(*input[0]).unwrap();
            tx1.send(0).unwrap();
            vm1.execute();
            let res = vm::last(&rx1);

            tx2.send(*input[1]).unwrap();
            tx2.send(res).unwrap();
            vm2.execute();
            let res = vm::last(&rx2);

            tx3.send(*input[2]).unwrap();
            tx3.send(res).unwrap();
            vm3.execute();
            let res = vm::last(&rx3);

            tx4.send(*input[3]).unwrap();
            tx4.send(res).unwrap();
            vm4.execute();
            let res = vm::last(&rx4);

            tx5.send(*input[4]).unwrap();
            tx5.send(res).unwrap();
            vm5.execute();
            let res = vm::last(&rx5);
            max = max.max(res);
        }

        Ok(max.into())        
    }

    fn run_part2(&mut self, _input: &[String]) -> Result<AoCResult> {
        Ok(AoCResult::None)
    }
}