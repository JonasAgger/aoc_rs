use anyhow::Result;
use point::IPoint;

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
        let machines: Vec<_> = slice_utils::split_chunk_empty(input)
            .into_iter()
            .map(ClawMachine::parse)
            .collect();

        let sum: usize = machines
            .into_iter()
            .map(|machine| machine.cramers_rule_score())
            .sum();
        Ok(sum.into())
    }

    fn run_part2(&mut self, input: &[String]) -> Result<AoCResult> {
        const OFFSET: i64 = 10000000000000;

        let machines: Vec<_> = slice_utils::split_chunk_empty(input)
            .into_iter()
            .map(ClawMachine::parse)
            .map(|mut machine| {
                machine.price = machine.price.add_x(OFFSET).unwrap();
                machine.price = machine.price.add_y(OFFSET).unwrap();

                machine
            })
            .collect();

        let sum: usize = machines
            .into_iter()
            .map(|machine| machine.cramers_rule_score())
            .sum();
        Ok(sum.into())
    }
}

#[derive(Debug)]
struct ClawMachine {
    button_a: IPoint,
    button_b: IPoint,

    price: IPoint,
}

impl ClawMachine {
    fn parse(s: Vec<String>) -> Self {
        ClawMachine {
            button_a: point(&s[0], '+'),
            button_b: point(&s[1], '+'),
            price: point(&s[2], '='),
        }
    }

    // https://www.youtube.com/watch?v=jBsC34PxzoM
    fn cramers_rule_score(&self) -> usize {
        // basically we have an input vector, which is: | button_a.x, button_b.x | mul | x |   | price.x |
        //                                              | button_a.y, button_b.y |  X  | y | = | price.y |

        // DET = (a_x*b_y - a_y*b_x)
        // A = (p_x*b_y - p_y*b_x) / DET
        // B = (a_x*p_y - a_y*p_x) / DET

        let determinant =
            self.button_a.x() * self.button_b.y() - self.button_a.y() * self.button_b.x();

        let a = self.price.x() * self.button_b.y() - self.price.y() * self.button_b.x();
        let b = self.button_a.x() * self.price.y() - self.button_a.y() * self.price.x();

        let a_det = a / determinant;
        let b_det = b / determinant;

        let p = IPoint::new(
            self.button_a.x() * a_det + self.button_b.x() * b_det,
            self.button_a.y() * a_det + self.button_b.y() * b_det,
        );

        // If it has a solution, return that.
        // a button cost 3, b button cost 1
        if p == self.price {
            (a_det * 3 + b_det) as usize
        } else {
            0
        }
    }
}

fn point(p_str: &str, delim: char) -> IPoint {
    let parts: Vec<i64> = p_str
        .split(delim)
        .skip(1)
        .map(|s| {
            if let Some(end) = s.find(',') {
                s[..end].parse().unwrap()
            } else {
                s.parse().unwrap()
            }
        })
        .collect();

    IPoint::new(parts[0], parts[1])
}
