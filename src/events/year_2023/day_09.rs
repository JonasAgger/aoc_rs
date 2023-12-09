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
    fn run_part1(&mut self, input: &Vec<String>) -> Result<AoCResult> {
        let sum: i64 = input
            .iter()
            .map(|line| {
                line.split_ascii_whitespace()
                    .filter_map(|nr| nr.parse().ok())
                    .collect::<Vec<i64>>()
            })
            .map(|oasis| generate_next(oasis))
            .sum();

        Ok(sum.into())
    }

    fn run_part2(&mut self, input: &Vec<String>) -> Result<AoCResult> {
        let sum: i64 = input
            .iter()
            .map(|line| {
                line.split_ascii_whitespace()
                    .filter_map(|nr| nr.parse().ok())
                    .collect::<Vec<i64>>()
            })
            .map(|oasis| generate_prev(oasis))
            .sum();

        Ok(sum.into())
    }
}

fn generate_next(sequence: Vec<i64>) -> i64 {
    let mut sequences = vec![sequence];

    while !is_all_same(sequences.last().unwrap()) {
        let mut new_sequence = vec![];
        let work = sequences.last().unwrap();
        for i in 1..work.len() {
            new_sequence.push(work[i] - work[i - 1]);
        }
        sequences.push(new_sequence);
    }

    let mut step = 0;
    for seq in sequences.iter().rev() {
        step = seq.last().unwrap() + step;
    }

    step
}

fn generate_prev(sequence: Vec<i64>) -> i64 {
    let mut sequences = vec![sequence];

    while !is_all_same(sequences.last().unwrap()) {
        let mut new_sequence = vec![];
        let work = sequences.last().unwrap();
        for i in 1..work.len() {
            new_sequence.push(work[i] - work[i - 1]);
        }
        sequences.push(new_sequence);
    }

    let mut step = 0;
    for seq in sequences.iter().rev() {
        step = seq.first().unwrap() - step;
    }

    step
}

fn is_all_same(sequence: &Vec<i64>) -> bool {
    let element = sequence[0];
    sequence.iter().all(|i| i.eq(&element))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn generate_next_test() {
        let input: Vec<i64> = vec![10, 13, 16, 21, 30, 45];

        let next = generate_next(input);
        assert_eq!(next, 68)
    }

    #[test]
    fn generate_prev_test() {
        let input: Vec<i64> = vec![10, 13, 16, 21, 30, 45];

        let next = generate_prev(input);
        assert_eq!(next, 5)
    }
}
