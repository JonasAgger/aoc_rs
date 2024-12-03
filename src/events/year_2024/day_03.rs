use anyhow::Result;
use regex::Regex;

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
        let rx = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")?;

        let sum: usize = input
            .into_iter()
            .map(|s| {
                let mut sum = 0;
                for (_, [nr1, nr2]) in rx.captures_iter(s).map(|c| c.extract()) {
                    let nr1: usize = nr1.number();
                    let nr2: usize = nr2.number();

                    sum += nr1 * nr2;
                }
                sum
            })
            .sum();

        Ok(sum.into())
    }

    fn run_part2(&mut self, input: &[String]) -> Result<AoCResult> {
        let rx = Regex::new(r"((do|don't)\(\))|(mul\((\d{1,3}),(\d{1,3})\))")?;
        let mut enabled = true;

        let sum: usize = input
            .into_iter()
            .map(|s| {
                let mut sum = 0;
                for cap in rx.captures_iter(s) {
                    let enable = cap.get(2);

                    match enable {
                        Some(x) => {
                            enabled = x.as_str() == "do";
                            continue;
                        }
                        None => (),
                    };

                    let nr1: usize = cap.get(4).map(|s| s.as_str().number()).unwrap();
                    let nr2: usize = cap.get(5).map(|s| s.as_str().number()).unwrap();

                    if enabled {
                        sum += nr1 * nr2;
                    }
                }
                sum
            })
            .sum();

        Ok(sum.into())
    }
}
