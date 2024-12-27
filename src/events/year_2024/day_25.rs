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
        let (locks, keys) = parse(input);

        let sum: usize = locks.into_iter().map(|lock| fits(lock, &keys)).sum();
        Ok(sum.into())
    }

    fn run_part2(&mut self, input: &[String]) -> Result<AoCResult> {
        AoCResult::initial_black_box(input)
    }
}

fn fits(lock: Lock, keys: &[Key]) -> usize {
    keys.iter()
        .filter(|&key| lock.fit(key))
        // .skip_while(|&key| !lock.fit(key))
        // .take_while(|&key| lock.fit(key))
        .count()
}

fn parse(input: &[String]) -> (Vec<Lock>, Vec<Key>) {
    let mut keys = vec![];
    let mut locks = vec![];

    for input in split_chunk_empty(input) {
        let mut data = [0; 5];
        let is_lock = input[0] == "#####";
        let input = if is_lock {
            &input[1..]
        } else {
            &input[..(input.len() - 1)]
        };

        for line in input {
            for (i, c) in line.chars().enumerate() {
                if c == '#' {
                    data[i] += 1;
                }
            }
        }

        if is_lock {
            locks.push(Lock(data));
        } else {
            keys.push(Key(data));
        }
    }

    locks.sort();
    keys.sort();

    (locks, keys)
}

#[derive(Debug, PartialEq, Clone, Copy, PartialOrd, Eq, Ord)]
struct Key([u8; 5]);
#[derive(Debug, PartialEq, Clone, Copy, PartialOrd, Eq, Ord)]
struct Lock([u8; 5]);

impl Lock {
    fn fit(&self, key: &Key) -> bool {
        self.0.iter().zip(key.0.iter()).all(|(x, y)| x + y < 6)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let (locks, keys) = parse(&[
            "#####".into(),
            ".####".into(),
            ".####".into(),
            ".####".into(),
            ".#.#.".into(),
            ".#...".into(),
            ".....".into(),
            "".into(),
            ".....".into(),
            ".....".into(),
            ".....".into(),
            "#....".into(),
            "#.#..".into(),
            "#.#.#".into(),
            "#####".into(),
            "".into(),
            ".....".into(),
            ".....".into(),
            ".....".into(),
            "....#".into(),
            "..#.#".into(),
            "#.#.#".into(),
            "#####".into(),
        ]);

        assert_eq!(locks.len(), 1);
        assert_eq!(keys.len(), 2);

        assert_eq!(locks[0], Lock([0, 5, 3, 4, 3]));
        assert_eq!(keys[0], Key([1, 0, 2, 0, 3]));
    }

    #[test]
    fn test_fit() {
        let (mut locks, keys) = parse(&[
            "#####".into(),
            ".####".into(),
            ".####".into(),
            ".####".into(),
            ".#.#.".into(),
            ".#...".into(),
            ".....".into(),
            "".into(),
            ".....".into(),
            ".....".into(),
            ".....".into(),
            "#....".into(),
            "#.#..".into(),
            "#.#.#".into(),
            "#####".into(),
        ]);

        let lock = locks.pop().unwrap();

        assert!(lock.fit(&keys[0]))
    }
}
