use anyhow::Result;

use crate::utils::*;

use super::super::AocDay;

pub struct Day {}

impl Day {
    pub fn new() -> Self {
        Self {}
    }

    fn count_winning_occurences(&self, line: &str) -> usize {
        let mut parts = line.split(':').last().unwrap().split('|');

        let winning: Vec<usize> = parts
            .next()
            .expect("Expected winning numbers")
            .split_ascii_whitespace()
            .filter_map(|number| number.trim().parse().ok())
            .collect();
        let your_numbers: Vec<usize> = parts
            .last()
            .expect("Expected your numbers")
            .split_ascii_whitespace()
            .filter_map(|number| number.trim().parse().ok())
            .collect();

        your_numbers
            .iter()
            .filter(|nr| winning.contains(nr))
            .count()
    }
}

impl AocDay for Day {
    fn run_part1(&mut self, input: &[String]) -> Result<AoCResult> {
        let mut points = 0;

        for line in input {
            let occurences = self.count_winning_occurences(line) as u32;

            // 2^(1-1) = 1, 2^(2-1) = 2, 2^(3-1) = 4 etc.
            points += 2usize.pow(occurences - 1);
        }

        Ok(points.into())
    }

    fn run_part2(&mut self, input: &[String]) -> Result<AoCResult> {
        let card_win_index_map: Vec<_> = input
            .iter()
            .map(|line| self.count_winning_occurences(line))
            .collect();

        let mut card_counts = vec![1usize; input.len()];

        for i in 0..card_counts.len() {
            let card_count = card_counts[i];
            let wins_on_card = card_win_index_map[i];

            // Start at i + 1 and move to i + 1 + wins
            for j in (i + 1)..(i + wins_on_card + 1) {
                card_counts[j] += card_count;
            }
        }

        Ok(card_counts.iter().sum::<usize>().into())
    }
}
