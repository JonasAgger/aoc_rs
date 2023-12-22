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
        let mut instruction_set: Vec<i64> =
            input[0].split(',').filter_map(|p| p.parse().ok()).collect();
        Ok(AoCResult::None)
    }

    fn run_part2(&mut self, input: &[String]) -> Result<AoCResult> {
        Ok(AoCResult::None)
    }
}
