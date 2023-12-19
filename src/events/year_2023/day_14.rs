use core::panic;
use std::{collections::HashMap, fmt::Display};

use anyhow::Result;
use tracing::debug;

use crate::utils::{grid::Grid2D, *};

use super::super::AocDay;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash)]
enum Rock {
    Round,
    Cube,
    Empty,
}

impl Rock {
    fn parse(c: char) -> Self {
        match c {
            '#' => Rock::Cube,
            'O' => Rock::Round,
            _ => Rock::Empty,
        }
    }
}

impl Display for Rock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rock::Round => write!(f, "O"),
            Rock::Cube => write!(f, "#"),
            Rock::Empty => write!(f, "."),
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
        let mut grid = Grid2D::parse(input, |line| line.chars().map(Rock::parse).collect());

        tilt_north(&mut grid);

        debug!("{}", grid);
        let mut total_value = 0;

        for row in 0..grid.height() {
            let value = grid.height() - row;

            let round_count = grid
                .get_row(row)
                .into_iter()
                .filter(|item| **item == Rock::Round)
                .count();

            let row_value = round_count * value;
            total_value += row_value;
            debug!(
                "Row {} has {} round rocks with {} value each = {}",
                row, round_count, value, row_value
            );
        }

        Ok(total_value.into())
    }

    fn run_part2(&mut self, input: &[String]) -> Result<AoCResult> {
        const ITERATIONS: usize = 1_000_000_000;

        let mut grid = Grid2D::parse(input, |line| line.chars().map(Rock::parse).collect());
        let mut iteration = 0;
        let mut seen = HashMap::new();

        let mut expected_repeat = None;
        let mut repeat_times = 0;

        while iteration < ITERATIONS {
            tilt_north(&mut grid);
            tilt_west(&mut grid);
            tilt_south(&mut grid);
            tilt_east(&mut grid);

            let hash = grid.hash_state();

            if let Some((expected_next, last, diff)) = expected_repeat
                && iteration == expected_next
            {
                let hash_last = seen.entry(hash).or_default();

                if last != *hash_last {
                    println!("last: {} -- hash_last {}", last, hash_last);
                    panic!("assertion failed");
                }

                repeat_times += 1;
                if repeat_times > 5 {
                    // just checking that it actually repeats. Is not really needed

                    let remainder = ITERATIONS - iteration;
                    let post = remainder % diff - 1;

                    for _i in 0..post {
                        tilt_north(&mut grid);
                        tilt_west(&mut grid);
                        tilt_south(&mut grid);
                        tilt_east(&mut grid);
                    }

                    let mut total_value = 0;

                    for row in 0..grid.height() {
                        let value = grid.height() - row;

                        let round_count = grid
                            .get_row(row)
                            .into_iter()
                            .filter(|item| **item == Rock::Round)
                            .count();

                        let row_value = round_count * value;
                        total_value += row_value;
                        debug!(
                            "Row {} has {} round rocks with {} value each = {}",
                            row, round_count, value, row_value
                        );
                    }

                    return Ok(total_value.into());
                }

                expected_repeat = Some((iteration + diff, last, diff));
            }

            let entry = seen.entry(hash);

            match entry {
                // if we found a repeat, lets try to identify when it repeats next
                std::collections::hash_map::Entry::Occupied(last_seen) => {
                    if expected_repeat.is_none() {
                        let last = *last_seen.get();
                        let diff = iteration - last;
                        let next = iteration + diff;
                        expected_repeat = Some((next, last, diff));
                    }
                }
                std::collections::hash_map::Entry::Vacant(vacant) => {
                    vacant.insert(iteration);
                }
            };

            iteration += 1;
        }

        unreachable!()
    }
}

fn tilt_north(grid: &mut Grid2D<Rock>) {
    fn find_destination(grid: &&mut Grid2D<Rock>, current_row: usize, col_index: usize) -> usize {
        for row in (0..current_row).rev() {
            if *grid.get((col_index, row)).unwrap() != Rock::Empty {
                return row + 1;
            }
        }
        0
    }

    let rows = grid.height();
    for row_index in 1..rows {
        let row: Vec<_> = grid.get_row(row_index).into_iter().copied().collect();
        for (col_index, _) in row
            .iter()
            .enumerate()
            .filter(|(_, rock)| **rock == Rock::Round)
        {
            let destination_row = find_destination(&grid, row_index, col_index);

            let rock_origin = grid.get_mut((col_index, row_index)).unwrap();
            *rock_origin = Rock::Empty;

            let rock_destination = grid.get_mut((col_index, destination_row)).unwrap();
            *rock_destination = Rock::Round;
        }
    }
}

fn tilt_south(grid: &mut Grid2D<Rock>) {
    fn find_destination(grid: &&mut Grid2D<Rock>, current_row: usize, col_index: usize) -> usize {
        for row in current_row + 1..grid.height() {
            if *grid.get((col_index, row)).unwrap() != Rock::Empty {
                return row - 1;
            }
        }
        grid.height() - 1
    }

    let rows = grid.height();
    for row_index in (0..rows - 1).rev() {
        let row: Vec<_> = grid.get_row(row_index).into_iter().copied().collect();
        for (col_index, _) in row
            .iter()
            .enumerate()
            .filter(|(_, rock)| **rock == Rock::Round)
        {
            let destination_row = find_destination(&grid, row_index, col_index);

            let rock_origin = grid.get_mut((col_index, row_index)).unwrap();
            *rock_origin = Rock::Empty;

            let rock_destination = grid.get_mut((col_index, destination_row)).unwrap();
            *rock_destination = Rock::Round;
        }
    }
}

fn tilt_west(grid: &mut Grid2D<Rock>) {
    fn find_destination(grid: &&mut Grid2D<Rock>, current_col: usize, row_index: usize) -> usize {
        for col in (0..current_col).rev() {
            if *grid.get((col, row_index)).unwrap() != Rock::Empty {
                return col + 1;
            }
        }
        0
    }

    let cols = grid.width();
    for col_index in 1..cols {
        let col: Vec<_> = grid.get_col(col_index).into_iter().copied().collect();
        for (row_index, _) in col
            .iter()
            .enumerate()
            .filter(|(_, rock)| **rock == Rock::Round)
        {
            let destination_col = find_destination(&grid, col_index, row_index);

            let rock_origin = grid.get_mut((col_index, row_index)).unwrap();
            *rock_origin = Rock::Empty;

            let rock_destination = grid.get_mut((destination_col, row_index)).unwrap();
            *rock_destination = Rock::Round;
        }
    }
}

fn tilt_east(grid: &mut Grid2D<Rock>) {
    fn find_destination(grid: &&mut Grid2D<Rock>, current_col: usize, row_index: usize) -> usize {
        for col in current_col + 1..grid.width() {
            if *grid.get((col, row_index)).unwrap() != Rock::Empty {
                return col - 1;
            }
        }
        grid.width() - 1
    }

    let cols = grid.width();
    for col_index in (0..cols - 1).rev() {
        let col: Vec<_> = grid.get_col(col_index).into_iter().copied().collect();
        for (row_index, _) in col
            .iter()
            .enumerate()
            .filter(|(_, rock)| **rock == Rock::Round)
        {
            let destination_col = find_destination(&grid, col_index, row_index);

            let rock_origin = grid.get_mut((col_index, row_index)).unwrap();
            *rock_origin = Rock::Empty;

            let rock_destination = grid.get_mut((destination_col, row_index)).unwrap();
            *rock_destination = Rock::Round;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_edgecase_last() {
        let input: Vec<_> = vec![
            String::from("..."),
            String::from(".#."),
            String::from(".O."),
            String::from("..."),
            String::from(".O."),
        ];

        let mut grid = Grid2D::parse(&input, |s| s.chars().map(Rock::parse).collect());
        tilt_north(&mut grid);

        let found = grid.find_all(|r| *r == Rock::Round);

        assert_eq!(Point::new(1, 2), *found.first().unwrap(), "first");
        assert_eq!(Point::new(1, 3), *found.last().unwrap(), "last");
    }

    #[test]
    fn test_tilt_south() {
        let input: Vec<_> = vec![
            String::from("..."),
            String::from(".#."),
            String::from(".O."),
            String::from("..."),
            String::from(".O."),
        ];

        let mut grid = Grid2D::parse(&input, |s| s.chars().map(Rock::parse).collect());
        tilt_south(&mut grid);

        let found = grid.find_all(|r| *r == Rock::Round);

        assert_eq!(Point::new(1, 3), *found.first().unwrap(), "first");
        assert_eq!(Point::new(1, 4), *found.last().unwrap(), "last");
    }

    #[test]
    fn test_cycling() {
        let input = vec![
            "O....#....".to_string(),
            "O.OO#....#".to_string(),
            ".....##...".to_string(),
            "OO.#O....O".to_string(),
            ".O.....O#.".to_string(),
            "O.#..O.#.#".to_string(),
            "..O..#O..O".to_string(),
            ".......O..".to_string(),
            "#....###..".to_string(),
            "#OO..#....".to_string(),
        ];

        let north = vec![
            "OOOO.#.O..".to_string(),
            "OO..#....#".to_string(),
            "OO..O##..O".to_string(),
            "O..#.OO...".to_string(),
            "........#.".to_string(),
            "..#....#.#".to_string(),
            "..O..#.O.O".to_string(),
            "..O.......".to_string(),
            "#....###..".to_string(),
            "#....#....".to_string(),
        ];

        let west = vec![
            "OOOO.#O...".to_string(),
            "OO..#....#".to_string(),
            "OOO..##O..".to_string(),
            "O..#OO....".to_string(),
            "........#.".to_string(),
            "..#....#.#".to_string(),
            "O....#OO..".to_string(),
            "O.........".to_string(),
            "#....###..".to_string(),
            "#....#....".to_string(),
        ];

        let south = vec![
            ".....#....".to_string(),
            "....#.O..#".to_string(),
            "O..O.##...".to_string(),
            "O.O#......".to_string(),
            "O.O....O#.".to_string(),
            "O.#..O.#.#".to_string(),
            "O....#....".to_string(),
            "OO....OO..".to_string(),
            "#O...###..".to_string(),
            "#O..O#....".to_string(),
        ];

        let east = vec![
            ".....#....".to_string(),
            "....#...O#".to_string(),
            "...OO##...".to_string(),
            ".OO#......".to_string(),
            ".....OOO#.".to_string(),
            ".O#...O#.#".to_string(),
            "....O#....".to_string(),
            "......OOOO".to_string(),
            "#...O###..".to_string(),
            "#..OO#....".to_string(),
        ];

        let mut grid = Grid2D::parse(&input, |s| s.chars().map(Rock::parse).collect());
        let north_grid = Grid2D::parse(&north, |s| s.chars().map(Rock::parse).collect());
        let west_grid = Grid2D::parse(&west, |s| s.chars().map(Rock::parse).collect());
        let south_grid = Grid2D::parse(&south, |s| s.chars().map(Rock::parse).collect());
        let east_grid = Grid2D::parse(&east, |s| s.chars().map(Rock::parse).collect());

        tilt_north(&mut grid);
        assert_eq!(format!("{}", grid), format!("{}", north_grid), "north");
        tilt_west(&mut grid);
        assert_eq!(format!("{}", grid), format!("{}", west_grid), "west");
        tilt_south(&mut grid);
        assert_eq!(format!("{}", grid), format!("{}", south_grid), "south");
        tilt_east(&mut grid);
        assert_eq!(format!("{}", grid), format!("{}", east_grid), "east");
    }

    #[test]
    fn test_cycling_multiple() {
        let input = vec![
            "O....#....".to_string(),
            "O.OO#....#".to_string(),
            ".....##...".to_string(),
            "OO.#O....O".to_string(),
            ".O.....O#.".to_string(),
            "O.#..O.#.#".to_string(),
            "..O..#O..O".to_string(),
            ".......O..".to_string(),
            "#....###..".to_string(),
            "#OO..#....".to_string(),
        ];

        let cycle1 = vec![
            ".....#....".to_string(),
            "....#...O#".to_string(),
            "...OO##...".to_string(),
            ".OO#......".to_string(),
            ".....OOO#.".to_string(),
            ".O#...O#.#".to_string(),
            "....O#....".to_string(),
            "......OOOO".to_string(),
            "#...O###..".to_string(),
            "#..OO#....".to_string(),
        ];

        let cycle2 = vec![
            ".....#....".to_string(),
            "....#...O#".to_string(),
            ".....##...".to_string(),
            "..O#......".to_string(),
            ".....OOO#.".to_string(),
            ".O#...O#.#".to_string(),
            "....O#...O".to_string(),
            ".......OOO".to_string(),
            "#..OO###..".to_string(),
            "#.OOO#...O".to_string(),
        ];

        let cycle3 = vec![
            ".....#....".to_string(),
            "....#...O#".to_string(),
            ".....##...".to_string(),
            "..O#......".to_string(),
            ".....OOO#.".to_string(),
            ".O#...O#.#".to_string(),
            "....O#...O".to_string(),
            ".......OOO".to_string(),
            "#...O###.O".to_string(),
            "#.OOO#...O".to_string(),
        ];

        let mut grid = Grid2D::parse(&input, |s| s.chars().map(Rock::parse).collect());
        let c1 = Grid2D::parse(&cycle1, |s| s.chars().map(Rock::parse).collect());
        let c2 = Grid2D::parse(&cycle2, |s| s.chars().map(Rock::parse).collect());
        let c3 = Grid2D::parse(&cycle3, |s| s.chars().map(Rock::parse).collect());

        tilt_north(&mut grid);
        tilt_west(&mut grid);
        tilt_south(&mut grid);
        tilt_east(&mut grid);

        assert_eq!(format!("{}", grid), format!("{}", c1), "cycle1");

        tilt_north(&mut grid);
        tilt_west(&mut grid);
        tilt_south(&mut grid);
        tilt_east(&mut grid);

        assert_eq!(format!("{}", grid), format!("{}", c2), "cycle2");

        tilt_north(&mut grid);
        tilt_west(&mut grid);
        tilt_south(&mut grid);
        tilt_east(&mut grid);

        assert_eq!(format!("{}", grid), format!("{}", c3), "cycle3");
    }
}
