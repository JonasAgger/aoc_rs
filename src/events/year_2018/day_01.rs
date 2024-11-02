use std::collections::HashSet;

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
        let sum = input
            .iter()
            .filter_map(|s| s.trim().parse::<i64>().ok())
            .sum::<i64>();

        Ok(sum.into())
    }

    fn run_part2(&mut self, input: &[String]) -> Result<AoCResult> {
        let mut seen = HashSet::new();
        let values: Vec<i64> = input
            .iter()
            .filter_map(|s| s.trim().parse::<i64>().ok())
            .collect();

        let mut curr = 0;

        let mut idx = 0;

        while seen.insert(curr) {
            curr += values[idx];
            idx = (idx + 1) % values.len();
        }

        Ok(curr.into())
    }
}
