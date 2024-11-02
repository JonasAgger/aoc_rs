use anyhow::Result;

use crate::utils::*;

use self::slice_utils::GrpBy;

use super::super::AocDay;

pub struct Day {}

impl Day {
    pub fn new() -> Self {
        Self {}
    }
}

impl AocDay for Day {
    fn run_part1(&mut self, input: &[String]) -> Result<AoCResult> {
        let mut has_2 = 0;
        let mut has_3 = 0;

        for data in input {
            let mut c: Vec<_> = data.chars().collect();
            c.sort();

            let grps: Vec<_> = c.group_by(|a, b| a == b).collect();
            if grps.iter().any(|x| x.len() == 2) {
                has_2 += 1;
            }
            if grps.iter().any(|x| x.len() == 3) {
                has_3 += 1;
            }
        }

        Ok((has_2 * has_3).into())
    }

    fn run_part2(&mut self, _: &[String]) -> Result<AoCResult> {
        Ok(AoCResult::None)
    }
}

fn cmp(a: &str, b: &str) -> bool {
    let mut found_one = false;
    for c in 0..a.len() {
        if a.as_bytes()[c] != b.as_bytes()[c] {
            if !found_one {
                found_one = true;
            } else {
                return false;
            }
        }
    }

    true
}
