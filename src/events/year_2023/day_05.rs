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
        let mut seeds: Vec<usize> = input[0]
            .split_once(':')
            .unwrap()
            .1
            .split_ascii_whitespace()
            .filter_map(|item| item.trim().parse().ok())
            .collect();

        let mut maps = vec![];

        let mut start = 3;
        for i in 3..input.len() {
            if input[i].eq("") {
                maps.push(Map::new(&input[start..i]));
                start = i + 2;
            }
        }

        maps.push(Map::new(&input[start..]));

        for map in maps.iter() {
            for seed in seeds.iter_mut() {
                *seed = map.get_index(*seed);
            }
        }

        Ok(seeds.iter().min().unwrap().to_owned().into())
    }

    fn run_part2(&mut self, input: &[String]) -> Result<AoCResult> {
        let seeds: Vec<usize> = input[0]
            .split_once(':')
            .unwrap()
            .1
            .split_ascii_whitespace()
            .filter_map(|item| item.trim().parse().ok())
            .collect();

        let mut maps = vec![];

        let mut start = 3;
        for i in 3..input.len() {
            if input[i].eq("") {
                maps.push(Map::new(&input[start..i]));
                start = i + 2;
            }
        }

        maps.push(Map::new(&input[start..]));

        let seeds = std::sync::Arc::new(seeds);
        let maps = std::sync::Arc::new(maps);

        let mins: Vec<_> = (0..seeds.len())
            .step_by(2)
            .map(|i| {
                let seeds = seeds.clone();
                let maps = maps.clone();
                std::thread::spawn(move || {
                    let range = seeds[i]..(seeds[i] + seeds[i + 1]);
                    let mut min_seed = usize::MAX;

                    for seed in range {
                        let mut seed = seed;
                        for map in maps.iter() {
                            seed = map.get_index(seed);
                        }
                        min_seed = min_seed.min(seed);
                    }

                    min_seed
                })
            })
            .collect();

        let min_seed = mins.into_iter().map(|t| t.join().unwrap()).min().unwrap();

        Ok(min_seed.into())
    }
}

#[derive(Debug)]
struct Map {
    source_ranges: Vec<std::ops::Range<usize>>,
    dest_ranges: Vec<std::ops::Range<usize>>,
}

impl Map {
    fn new(ranges: &[String]) -> Self {
        let mut source_ranges = vec![];
        let mut dest_ranges = vec![];

        for range in ranges.iter() {
            let parts: Vec<usize> = range
                .split_ascii_whitespace()
                .filter_map(|item| item.trim().parse().ok())
                .collect();
            dest_ranges.push(parts[0]..parts[0] + parts[2]);
            source_ranges.push(parts[1]..parts[1] + parts[2]);
        }

        Self {
            source_ranges,
            dest_ranges,
        }
    }

    pub fn get_index(&self, nr: usize) -> usize {
        match self
            .source_ranges
            .iter()
            .enumerate()
            .find(|&(_, r)| r.contains(&nr))
        {
            Some((index, range)) => {
                let from_start = nr - range.start;

                self.dest_ranges[index].start + from_start
            }
            None => nr,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn map_parsing() {
        let input = vec![String::from("50 98 2"), String::from("52 50 48")];

        let map: Map = Map::new(&input);

        assert_eq!(map.get_index(0), 0);
        assert_eq!(map.get_index(49), 49);
        assert_eq!(map.get_index(50), 52);
        assert_eq!(map.get_index(98), 50);
        assert_eq!(map.get_index(99), 51);
    }
}
