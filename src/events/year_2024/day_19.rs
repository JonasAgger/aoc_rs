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

        let mut cache = parse_towel(towels.into_iter().single(), true);

        let patterns = patterns.into_iter().map(|s| s.into_bytes());

        let (min, max) = cache.keys().map(|s| s.len()).min_max();

        let len = patterns
            .into_iter()
            .filter(|s| fits(s, min, max, &mut cache))
            .count();

        Ok(len.into())
    }

    fn run_part2(&mut self, input: &[String]) -> Result<AoCResult> {
        AoCResult::initial_black_box(input)
    }
}

fn fits(pattern: &[u8], min: usize, max: usize, cache: &mut HashMap<Vec<u8>, bool>) -> bool {
    if pattern.len() == 0 {
        return true;
    }

    if let Some(value) = cache.get(pattern) {
        return *value;
    } else {
        for pattern_length in (min..=max).rev().filter(|&len| pattern.len() >= len) {
            let slice = &pattern[..pattern_length];

            if slice == pattern {
                continue;
            }

            if fits(slice, min, max, cache) && fits(&pattern[pattern_length..], min, max, cache) {
                cache.insert(pattern.to_vec(), true);
                return true;
            }
        }

        cache.insert(pattern.to_vec(), false);
        return false;
    }
}

fn permutations(
    pattern: &[u8],
    min: usize,
    max: usize,
    cache: &mut HashMap<Vec<u8>, usize>,
) -> usize {
    if pattern.len() == 0 {
        return 0;
    }

    if let Some(value) = cache.get(pattern) {
        return *value;
    } else {
        let mut count = 0;
        for pattern_length in (min..=max).rev().filter(|&len| pattern.len() >= len) {
            let slice = &pattern[..pattern_length];

            if slice == pattern {
                continue;
            }

            let left = permutations(slice, min, max, cache);
            let right = permutations(slice, min, max, cache);
            if left > 0 && right > 0 {
                count += left;
            }
        }

        cache.insert(pattern.to_vec(), count);
        return count;
    }
}

fn parse_towel<S: AsRef<str>, T: Copy>(s: S, default: T) -> HashMap<Vec<u8>, T> {
    s.as_ref()
        .split_ascii_whitespace()
        .map(|s| s.trim_matches(',').as_bytes().to_vec())
        .map(|x| (x, default))
        .collect()
}

fn seed(cache: HashMap<Vec<u8>, usize>) -> HashMap<Vec<u8>, usize> {
    let (min, max) = cache.keys().map(|s| s.len()).min_max();

    let mut new_cache = HashMap::new();

    for length in (min..=max) {
        let keys = cache.keys().filter(|key| key.len() == length);

        for key in keys {
            let mut count = 1; // add self

            count += permutations(&key, min, length, &mut new_cache);
            new_cache.insert(key.to_vec(), count);
        }
    }

    new_cache
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wat() {
        let mut cache = parse_towel("r, wr, b, g, bwu, rb, gb, br", true);

        let (min, max) = cache.keys().map(|s| s.len()).min_max();

        assert!(fits(b"brwrr", min, max, &mut cache))
    }

    #[test]
    fn wat2() {
        let mut cache = seed(parse_towel("r, wr, b, g, bwu, rb, gb, br", 1));

        let (min, max) = cache.keys().map(|s| s.len()).min_max();

        // assert_eq!(permutations(b"br", min, max, &mut cache), 2)
        assert_eq!(permutations(b"rrbgbr", min, max, &mut cache), 6)
    }
}
