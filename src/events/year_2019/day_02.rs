use anyhow::Result;

use crate::utils::*;

use super::{super::AocDay, VM};

pub struct Day {}

impl Day {
    pub fn new() -> Self {
        Self {}
    }
}

impl AocDay for Day {
    fn run_part1(&mut self, input: &[String]) -> Result<AoCResult> {
        let mut instruction_set: Vec<i64> =
            input[0].split(',').filter_map(|p| p.parse().ok()).collect();

        instruction_set[1] = 12;
        instruction_set[2] = 2;

        let mut vm = VM::new(instruction_set);
        vm.execute();

        Ok(vm.get_value_at(0).into())
    }

    fn run_part2(&mut self, input: &[String]) -> Result<AoCResult> {
        let mut instruction_set: Vec<i64> =
            input[0].split(',').filter_map(|p| p.parse().ok()).collect();

        instruction_set[1] = 12;
        instruction_set[2] = 2;

        let mut vm = VM::new(instruction_set);
        vm.execute();

        Ok(vm.get_value_at(0).into())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn testcase1() {
        let input: Vec<i64> = vec![1, 0, 0, 0, 99];

        let expected: Vec<i64> = vec![2, 0, 0, 0, 99];

        let mut vm = VM::new(input);

        vm.execute();

        assert_eq!(expected, vm.instructions())
    }
    #[test]
    fn testcase2() {
        let input: Vec<i64> = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];

        let expected: Vec<i64> = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];

        let mut vm = VM::new(input);

        vm.execute();

        assert_eq!(expected, vm.instructions())
    }

    #[test]
    fn testcase3() {
        let input: Vec<i64> = vec![2, 3, 0, 3, 99];

        let expected: Vec<i64> = vec![2, 3, 0, 6, 99];

        let mut vm = VM::new(input);

        vm.execute();

        assert_eq!(expected, vm.instructions())
    }

    #[test]
    fn testcase4() {
        let input: Vec<i64> = vec![2, 4, 4, 5, 99, 0];

        let expected: Vec<i64> = vec![2, 4, 4, 5, 99, 9801];

        let mut vm = VM::new(input);

        vm.execute();

        assert_eq!(expected, vm.instructions())
    }
}
