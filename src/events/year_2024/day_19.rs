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
        let [towels, patterns]: [Vec<String>; 2] = split_chunk_empty(input).try_into().unwrap();

        let mut cache = Default::default();
        let towels = parse_towel(towels.into_iter().single());

        let patterns = patterns.into_iter().map(|s| s.into_bytes());

        let len = patterns
            .into_iter()
            .filter(|s| permutations(&s, &towels, &mut cache) > 0)
            .count();

        Ok(len.into())
    }

    fn run_part2(&mut self, input: &[String]) -> Result<AoCResult> {
        let [towels, patterns]: [Vec<String>; 2] = split_chunk_empty(input).try_into().unwrap();

        let mut cache = Default::default();
        let towels = parse_towel(towels.into_iter().single());

        let patterns = patterns.into_iter().map(|s| s.into_bytes());

        let permutations: usize = patterns
            .into_iter()
            .map(|s| permutations(&s, &towels, &mut cache))
            .sum();

        Ok(permutations.into())
    }
}

fn permutations(pattern: &[u8], towels: &[Vec<u8>], cache: &mut HashMap<Vec<u8>, usize>) -> usize {
    if pattern.len() == 0 {
        1
    } else if let Some(&value) = cache.get(pattern) {
        value
    } else {
        let value = towels
            .iter()
            .filter(|key| pattern.starts_with(key))
            .map(|key| permutations(pattern.strip_prefix(key.as_slice()).unwrap(), towels, cache))
            .sum();
        cache.insert(pattern.to_vec(), value);
        value
    }
}

fn parse_towel<S: AsRef<str>>(s: S) -> Vec<Vec<u8>> {
    s.as_ref()
        .split_ascii_whitespace()
        .map(|s| s.trim_matches(',').as_bytes().to_vec())
        .collect()
}
