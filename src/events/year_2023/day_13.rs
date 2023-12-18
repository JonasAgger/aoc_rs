use std::fmt::Display;

use anyhow::Result;

use crate::utils::{*, grid::Grid2D};

use super::super::AocDay;

#[derive(Debug, PartialEq, PartialOrd)]
enum RowCol {
    Row(usize),
    Col(usize)
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum Env {
    Ash,
    Rock
}

impl Env {
    pub fn parse(c: char) -> Self {
        match c {
            '#' => Env::Rock,
            _ => Env::Ash
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

pub struct Day {

}

impl Day {
    pub fn new() -> Self {
        Self {}
    }
}

impl AocDay for Day {
    fn run_part1(&mut self, input: &[String]) -> Result<AoCResult> {

        let parts = slice_utils::split_chunk_empty(input);
        let grids: Vec<_> = parts.iter().map(|part| Grid2D::parse(part, |s| s.chars().map(Env::parse).collect())).collect();

        let mut result = 0;

        for grid in grids {
            println!("{}", &grid);
            let refection = find_reflection(&grid);
            result += match refection {
                RowCol::Row(val) => val * 100,
                RowCol::Col(val) => val,
            };
            println!("{:?}", refection);
        }

        Ok(result.into())
    }

    fn run_part2(&mut self, input: &[String]) -> Result<AoCResult> {
        Ok(AoCResult::None)
    }
}

fn find_reflection(grid: &Grid2D<Env>) -> RowCol {

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

fn matches(left: &[usize], right: &[usize]) -> bool {
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
    fn generate_next_test() {
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

        let reflection = find_reflection(&grid);

        assert_eq!(reflection, RowCol::Row(12))
    }
}
