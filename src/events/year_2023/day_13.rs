use std::fmt::Display;

use anyhow::Result;
use tracing::debug;

use crate::utils::{grid::Grid2D, *};

use super::super::AocDay;

#[derive(Debug, PartialEq, PartialOrd)]
enum RowCol {
    Row(usize),
    Col(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum Env {
    Ash,
    Rock,
}

impl Env {
    pub fn parse(c: char) -> Self {
        match c {
            '#' => Env::Rock,
            _ => Env::Ash,
        }
    }
}

impl Display for Env {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Env::Ash => write!(f, "."),
            Env::Rock => write!(f, "#"),
        }
    }
}

pub struct Day {}

impl Day {
    pub fn new() -> Self {
        Self {}
    }
}

impl AocDay for Day {
    fn run_part1(&mut self, input: &[String]) -> Result<AoCResult> {
        let parts = slice_utils::split_chunk_empty(input);
        let grids: Vec<_> = parts
            .iter()
            .map(|part| Grid2D::parse(part, |s| s.chars().map(Env::parse).collect()))
            .collect();

        let mut result = 0;

        for grid in grids {
            debug!("{}", &grid);
            let refection = find_reflection(&grid, matches_part1);
            result += match refection {
                RowCol::Row(val) => val * 100,
                RowCol::Col(val) => val,
            };
            debug!("{:?}", refection);
        }

        Ok(result.into())
    }

    fn run_part2(&mut self, input: &[String]) -> Result<AoCResult> {
        let parts = slice_utils::split_chunk_empty(input);
        let grids: Vec<_> = parts
            .iter()
            .map(|part| Grid2D::parse(part, |s| s.chars().map(Env::parse).collect()))
            .collect();

        let mut result = 0;

        for grid in grids {
            debug!("{}", &grid);
            let refection = find_reflection(&grid, matches_part2);
            result += match refection {
                RowCol::Row(val) => val * 100,
                RowCol::Col(val) => val,
            };
            debug!("{:?}", refection);
        }

        Ok(result.into())
    }
}

fn find_reflection<M: Fn(&[usize], &[usize]) -> bool>(grid: &Grid2D<Env>, matches: M) -> RowCol {
    let col_values = get_col_values(grid);
    let row_values = get_row_values(grid);

    for i in 1..col_values.len() {
        // if i-1 and i is equal, then check i-2 and i+1 and then i-3 and i+2
        if matches(&col_values[..i], &col_values[i..]) {
            return RowCol::Col(i);
        }
    }

    for i in 1..row_values.len() {
        // if i-1 and i is equal, then check i-2 and i+1 and then i-3 and i+2
        if matches(&row_values[..i], &row_values[i..]) {
            return RowCol::Row(i);
        }
    }

    panic!("wat");
}

fn matches_part1(left: &[usize], right: &[usize]) -> bool {
    let min_length = left.len().min(right.len());

    for i in 0..min_length {
        let left_index = left.len() - i - 1;
        let right_index = i;
        if left[left_index] != right[right_index] {
            return false;
        }
    }

    true
}

fn matches_part2(left: &[usize], right: &[usize]) -> bool {
    let min_length = left.len().min(right.len());
    let mut diffs = 0;

    for i in 0..min_length {
        let left_index = left.len() - i - 1;
        let right_index = i;

        // if diff is a full power of 10, then we accept it.
        // otherwise mark as failed.
        // we can only have 1 diff for the thing to work

        let diff = left[left_index].abs_diff(right[right_index]);

        if diff == 0 {
            continue;
        }

        if !is_power_of_10_diff(diff) {
            return false;
        }

        diffs += 1;
    }

    diffs == 1
}

fn is_power_of_10_diff(diff: usize) -> bool {
    let mut diff = diff;

    while diff >= 10 && diff % 10 == 0 {
        diff /= 10;
    }

    diff == 1
}

fn get_col_values(grid: &Grid2D<Env>) -> Vec<usize> {
    let mut values = vec![];

    for i in 0..grid.width() {
        let mut value = 0;
        for (index, &env_value) in grid.get_col(i).into_iter().enumerate() {
            if env_value == Env::Rock {
                value += 10_usize.pow(index as u32);
            }
        }
        values.push(value);
    }
    values
}

fn get_row_values(grid: &Grid2D<Env>) -> Vec<usize> {
    let mut values = vec![];

    for i in 0..grid.height() {
        let mut value = 0;
        for (index, &env_value) in grid.get_row(i).into_iter().enumerate() {
            if env_value == Env::Rock {
                value += 10_usize.pow(index as u32);
            }
        }
        values.push(value);
    }
    values
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_edgecase_last() {
        let input: Vec<_> = vec![
            String::from("..#..#..##..#"),
            String::from("##.#..##..##."),
            String::from("..#.###.####."),
            String::from(".#.#.#...##.."),
            String::from(".##.#.#..#.##"),
            String::from("#####..#....."),
            String::from("#####..#....."),
            String::from(".##.#.#..#.##"),
            String::from(".#.#.#..###.."),
            String::from("..#.###.####."),
            String::from("##.#..##..##."),
            String::from("..#..#..##..#"),
            String::from("..#..#..##..#"),
        ];

        let grid = Grid2D::parse(&input, |s| s.chars().map(Env::parse).collect());

        let reflection = find_reflection(&grid, matches_part1);

        assert_eq!(reflection, RowCol::Row(12))
    }

    #[test]
    fn test_part2_diff() {
        let input: Vec<_> = vec![
            "#.##..##.".to_string(),
            "..#.##.#.".to_string(),
            "##......#".to_string(),
            "##......#".to_string(),
            "..#.##.#.".to_string(),
            "..##..##.".to_string(),
            "#.#.##.#.".to_string(),
        ];

        let grid = Grid2D::parse(&input, |s| s.chars().map(Env::parse).collect());

        let row_values = get_row_values(&grid);

        let matches = matches_part2(&row_values[..3], &row_values[3..]);

        assert!(matches)
    }

    #[test]
    fn test_part2_diff2() {
        let input: Vec<_> = vec![
            "#...##..#".to_string(),
            "#....#..#".to_string(),
            "..##..###".to_string(),
            "#####.##.".to_string(),
            "#####.##.".to_string(),
            "..##..###".to_string(),
            "#....#..#".to_string(),
        ];

        let grid = Grid2D::parse(&input, |s| s.chars().map(Env::parse).collect());

        let row_values = find_reflection(&grid, matches_part2);

        assert_eq!(row_values, RowCol::Row(1))
    }

    #[test]
    fn is_power_of_10_tests() {
        let cases: Vec<(usize, bool)> = vec![
            (1, true),
            (2, false),
            (10, true),
            (11, false),
            (100, true),
            (1000, true),
        ];

        for (input, expected) in cases {
            let actual = is_power_of_10_diff(input);
            assert_eq!(
                actual, expected,
                "Checking if {} is a power of 10, expected: {}",
                input, expected
            );
        }
    }
}
