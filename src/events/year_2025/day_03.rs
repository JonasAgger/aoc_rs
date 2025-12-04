use anyhow::Result;

use crate::utils::*;

use super::super::AocDay;

pub struct Day {}

impl Day {
    pub fn new() -> Self {
        Self {}
    }
}

impl AocDay for Day {
    fn run_part1(&mut self, input: &[String]) -> Result<AoCResult> {
        let res: usize = input
            .into_iter()
            .map(|digits| {
                let len = digits.len();
                let mut first = 0;
                let mut last = 0;

                for digit in digits[..(len - 1)].as_bytes().iter().map(|b| b - b'0') {
                    if digit > first {
                        first = digit;
                        last = 0;
                    } else if last < digit {
                        last = digit;
                    }
                }
                last = last.max(digits.as_bytes()[len - 1] - b'0');

                (first * 10 + last) as usize
            })
            .sum();
        Ok(res.into())
    }

    fn run_part2(&mut self, input: &[String]) -> Result<AoCResult> {
        const NUMBERS: usize = 12;

        let res: usize = input
            .into_iter()
            .map(|digits| {
                let mut idx = 0;
                let len = digits.len();

                let mut result = 0;

                for remainder in (0..NUMBERS).rev() {
                    let end = len - remainder;
                    idx += find_max(&digits[idx..end]);

                    let value = digits.as_bytes()[idx] - b'0';

                    let index_value = 10usize.pow(remainder as u32) * value as usize;
                    result += index_value;

                    idx += 1; // we need to move forward at least 1
                }

                result
            })
            .sum();
        Ok(res.into())
    }
}

fn find_max(input: &str) -> usize {
    let mut max = 0;
    let mut max_idx = 0;

    for (idx, number) in input.as_bytes().iter().map(|b| b - b'0').enumerate() {
        if number == 9 {
            return idx;
        }

        if max < number {
            max = number;
            max_idx = idx;
        }
    }
    max_idx
}
