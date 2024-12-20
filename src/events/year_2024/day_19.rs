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
        let towels: Vec<Vec<u8>> = parse_towel(towels.into_iter().single());

        let patterns = patterns.into_iter().map(|s| s.into_bytes());

        let (min, max) = towels.iter().map(|s| s.len()).min_max();

        let len = patterns
            .into_iter()
            .filter(|s| fits(s, towels.as_slice(), min, max))
            .count();

        Ok(len.into())
    }

    fn run_part2(&mut self, input: &[String]) -> Result<AoCResult> {
        AoCResult::initial_black_box(input)
    }
}

fn fits(pattern: &[u8], towels: &[Vec<u8>], min: usize, max: usize) -> bool {
    // println!("{:?}", std::str::from_utf8(pattern));
    if pattern.len() <= max {
        if towels.iter().any(|s| s.as_slice() == pattern) {
            return true;
        }
    }

    for pattern_length in (min..max).rev().filter(|&len| pattern.len() >= len) {
        let slice = &pattern[..pattern_length];

        if fits(slice, towels, min, max) && fits(&pattern[pattern_length..], towels, min, max) {
            return true;
        }
    }

    false
}

fn parse_towel<S: AsRef<str>>(s: S) -> Vec<Vec<u8>> {
    s.as_ref()
        .split_ascii_whitespace()
        .map(|s| s.trim_matches(',').as_bytes().to_vec())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wat() {
        let towels: Vec<_> = parse_towel("r, wr, b, g, bwu, rb, gb, br");

        let (min, max) = towels.iter().map(|s| s.len()).min_max();

        assert!(fits(b"brwrr", &towels, min, max))
    }

    #[test]
    fn wat2() {
        let towels: Vec<_> = parse_towel("r, wr, b, g, bwu, rb, gb, br");

        let (min, max) = towels.iter().map(|s| s.len()).min_max();

        assert!(fits(b"rrbgbr", &towels, min, max))
    }
}
