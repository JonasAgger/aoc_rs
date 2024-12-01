use std::collections::HashMap;

use anyhow::Result;
use slice_utils::GrpBy;

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
        let (mut l1, mut l2): (Vec<i64>, Vec<i64>) = input
            .into_iter()
            .map(|x| {
                let mut x = x.split_ascii_whitespace();
                let x: (i64, i64) = (x.next().unwrap().number(), x.next().unwrap().number());
                x
            })
            .unzip();

        l1.sort();
        l2.sort();

        let sum: u64 = l1
            .into_iter()
            .zip(l2.into_iter())
            .map(|(x, y)| x.abs_diff(y))
            .sum();

        Ok(sum.into())
    }

    fn run_part2(&mut self, input: &[String]) -> Result<AoCResult> {
        let (mut l1, mut l2): (Vec<i64>, Vec<i64>) = input
            .into_iter()
            .map(|x| {
                let mut x = x.split_ascii_whitespace();
                let x: (i64, i64) = (x.next().unwrap().number(), x.next().unwrap().number());
                x
            })
            .unzip();
        l2.sort();

        let l2: HashMap<i64, usize> = l2
            .group_by(|x, y| x == y)
            .map(|x| (*x.first().unwrap(), x.len()))
            .collect();

        let sum: usize = l1
            .into_iter()
            .map(|x| x as usize * l2.get(&x).copied().unwrap_or(0usize))
            .sum();

        Ok(sum.into())
    }
}
