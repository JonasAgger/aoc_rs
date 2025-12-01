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
        let (_, times_at_zero) = input
            .into_iter()
            .map(|line| {
                let (way, number) = line.split_at(1);
                let number: isize = number.number();
                let number = match way {
                    "L" => number * -1,
                    _ => number,
                };
                number
            })
            .fold((50, 0), |(mut dial, mut times_at_zero), number| {
                dial += number % 100; // Numbers over 100 is the same as doing nothing
                // Reset overflows
                if dial < 0 {
                    dial += 100;
                } else if dial > 99 {
                    dial -= 100;
                }

                if dial == 0 {
                    times_at_zero += 1;
                }
                (dial, times_at_zero)
            });
        Ok(times_at_zero.into())
    }

    fn run_part2(&mut self, input: &[String]) -> Result<AoCResult> {
        let (_, times_at_zero) = input
            .into_iter()
            .map(|line| {
                let (way, number) = line.split_at(1);
                let number: isize = number.number();
                let way = match way {
                    "L" => -1,
                    _ => 1,
                };
                (way, number)
            })
            .fold((50, 0), |(mut dial, mut passed_zero), (way, number)| {
                let is_dial_at_zero = dial == 0;

                passed_zero += number / 100; // Count overflows

                let rotate_count = number % 100; // Numbers over 100 is the same as doing nothing and we've already counted them
                if rotate_count == 0 {
                    return (dial, passed_zero);
                }

                match way {
                    1 => {
                        dial += rotate_count;

                        if dial > 99 && !is_dial_at_zero {
                            passed_zero += 1;
                        }
                    }
                    -1 => {
                        dial += 100;
                        dial -= rotate_count;
                        if dial <= 100 && !is_dial_at_zero {
                            passed_zero += 1;
                        }
                    }
                    _ => unreachable!(),
                }

                (dial % 100, passed_zero)
            });
        Ok(times_at_zero.into())
    }
}
