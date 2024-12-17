use std::ops::BitXor;

use anyhow::Result;
use variable::{variable, Variable};

use crate::utils::*;

use super::super::AocDay;

const TEST: Variable<bool> = variable(true, false);

pub struct Day {}

impl Day {
    pub fn new() -> Self {
        Self {}
    }
}

impl AocDay for Day {
    fn run_part1(&mut self, _: &[String]) -> Result<AoCResult> {
        let mut computer = get_computer();
        computer.run();

        Ok(computer
            .output
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join(",")
            .into())
    }

    fn run_part2(&mut self, _: &[String]) -> Result<AoCResult> {
        let instructions = get_computer().instruction_cache;

        let reg_a = solve(&instructions, instructions.len(), 0).unwrap();

        Ok(reg_a.into())
    }
}

fn solve(instructions: &[u8], idx: usize, reg_a: i64) -> Option<i64> {
    // if we reach the end of the line, just return here. We've outputted all the numbers
    let Some(idx) = idx.checked_sub(1) else {
        return Some(reg_a);
    };

    let expected_output = instructions[idx];

    for i in 0..8 {
        let reg_a = reg_a << 3 | i;
        let mut computer = get_computer2(reg_a);
        while computer.forward().is_some() {}

        // at current iteration, we only need it to output the next value.
        if *computer.output.first().unwrap() == expected_output {
            if let Some(result) = solve(instructions, idx, reg_a) {
                return Some(result);
            }
        }
    }
    None
}

fn get_computer() -> Computer {
    if *TEST {
        Computer {
            instruction_cache: vec![0, 1, 5, 4, 3, 0],
            instruction_ptr: 0,
            reg_a: 729,
            reg_b: 0,
            reg_c: 0,
            output: vec![],
        }
    } else {
        Computer {
            instruction_cache: vec![2, 4, 1, 5, 7, 5, 0, 3, 4, 1, 1, 6, 5, 5, 3, 0],
            instruction_ptr: 0,
            reg_a: 47719761,
            reg_b: 0,
            reg_c: 0,
            output: vec![],
        }
    }
}

fn get_computer2(reg: i64) -> Computer {
    if *TEST {
        Computer {
            instruction_cache: vec![0, 3, 5, 4, 3, 0],
            instruction_ptr: 0,
            reg_a: reg,
            reg_b: 0,
            reg_c: 0,
            output: vec![],
        }
    } else {
        Computer {
            instruction_cache: vec![2, 4, 1, 5, 7, 5, 0, 3, 4, 1, 1, 6, 5, 5, 3, 0],
            instruction_ptr: 0,
            reg_a: reg,
            reg_b: 0,
            reg_c: 0,
            output: vec![],
        }
    }
}

#[derive(Debug)]
struct Computer {
    instruction_cache: Vec<u8>,
    instruction_ptr: usize,
    reg_a: i64,
    reg_b: i64,
    reg_c: i64,
    output: Vec<u8>,
}

impl Computer {
    fn run(&mut self) {
        while self.forward().is_some() {}
    }

    fn forward(&mut self) -> Option<()> {
        let opcode = *self.instruction_cache.get(self.instruction_ptr)?;
        let operand = *self.instruction_cache.get(self.instruction_ptr + 1)?;

        let to_move = match opcode {
            // adv - div
            0 => {
                let denominator = 2i64.pow(self.combo(operand) as u32);
                let sum = self.reg_a / denominator;
                self.reg_a = sum;

                2
            }
            // bxl - bitwise xor
            1 => {
                let result = self.reg_b.bitxor(operand as i64);
                self.reg_b = result;

                2
            }
            // bst
            2 => {
                self.reg_b = self.combo(operand) % 8;

                2
            }
            // jnz - jump
            3 => {
                if self.reg_a != 0 {
                    self.instruction_ptr = operand as usize;
                    0
                } else {
                    2
                }
            }
            // bxc - bitwise xor
            4 => {
                self.reg_b = self.reg_b.bitxor(self.reg_c);

                2
            }
            // out
            5 => {
                let output = self.combo(operand) % 8;
                self.output.push(output as u8);

                2
            }
            // bdv - div
            6 => {
                let denominator = 2i64.pow(self.combo(operand) as u32);
                let sum = self.reg_a / denominator;
                self.reg_b = sum;

                2
            }
            // cdv - div
            7 => {
                let denominator = 2i64.pow(self.combo(operand) as u32);
                let sum = self.reg_a / denominator;
                self.reg_c = sum;

                2
            }
            _ => unreachable!(),
        };

        self.instruction_ptr += to_move;

        Some(())
    }

    fn combo(&self, operand: u8) -> i64 {
        match operand {
            // literal
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,

            // regs
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,

            7 => unreachable!(),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_1() {
        let mut computer = Computer {
            instruction_cache: vec![2, 6],
            instruction_ptr: 0,
            reg_a: 0,
            reg_b: 0,
            reg_c: 9,
            output: vec![],
        };

        computer.run();

        assert_eq!(computer.reg_b, 1)
    }

    #[test]
    fn simple_2() {
        let mut computer = Computer {
            instruction_cache: vec![5, 0, 5, 1, 5, 4],
            instruction_ptr: 0,
            reg_a: 10,
            reg_b: 0,
            reg_c: 0,
            output: vec![],
        };

        computer.run();

        assert_eq!(computer.output, vec![0, 1, 2])
    }

    #[test]
    fn simple_3() {
        let mut computer = Computer {
            instruction_cache: vec![0, 1, 5, 4, 3, 0],
            instruction_ptr: 0,
            reg_a: 2024,
            reg_b: 0,
            reg_c: 0,
            output: vec![],
        };

        computer.run();

        assert_eq!(computer.reg_a, 0);
        assert_eq!(computer.output, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0])
    }

    #[test]
    fn simple_4() {
        let mut computer = Computer {
            instruction_cache: vec![1, 7],
            instruction_ptr: 0,
            reg_a: 0,
            reg_b: 29,
            reg_c: 0,
            output: vec![],
        };

        computer.run();

        assert_eq!(computer.reg_b, 26)
    }

    #[test]
    fn simple_5() {
        let mut computer = Computer {
            instruction_cache: vec![4, 0],
            instruction_ptr: 0,
            reg_a: 0,
            reg_b: 2024,
            reg_c: 43690,
            output: vec![],
        };

        computer.run();

        assert_eq!(computer.reg_b, 44354)
    }
}
