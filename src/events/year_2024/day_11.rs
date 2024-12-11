use std::collections::HashMap;

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
        const LEN: usize = 25;
        let sequence: Vec<usize> = input[0].split_ascii_whitespace().map(get_number).collect();

        let mut cache = HashMap::new();
        let sum: usize = sequence
            .into_iter()
            .map(|x| reduce(x, LEN, &mut cache))
            .sum();
        Ok(sum.into())
    }

    fn run_part2(&mut self, input: &[String]) -> Result<AoCResult> {
        const LEN: usize = 75;
        let sequence: Vec<usize> = input[0].split_ascii_whitespace().map(get_number).collect();

        let mut cache = HashMap::new();
        let sum: usize = sequence
            .into_iter()
            .map(|x| reduce(x, LEN, &mut cache))
            .sum();
        Ok(sum.into())
    }
}

#[derive(Hash, PartialEq, Eq)]
struct MemoKey {
    stone: usize,
    steps_remaining: usize,
}

fn reduce(stone: usize, steps_remaining: usize, cache: &mut HashMap<MemoKey, usize>) -> usize {
    // Cant replicate more
    if steps_remaining == 0 {
        return 1;
    }
    let key = MemoKey {
        stone,
        steps_remaining,
    };

    // check if seen before
    if let Some(&count) = cache.get(&key) {
        return count;
    }

    // recurse dat thing, given a max depth of 75 this should be fine
    let result = match stone {
        0 => reduce(1, steps_remaining - 1, cache),
        other => {
            let x = other.ilog10() + 1;
            if x % 2 == 0 {
                let left = other / 10usize.pow(x / 2);
                let right = other % 10usize.pow(x / 2);

                reduce(left, steps_remaining - 1, cache) + reduce(right, steps_remaining - 1, cache)
            } else {
                reduce(other * 2024, steps_remaining - 1, cache)
            }
        }
    };

    cache.insert(key, result);
    result
}
