use std::fmt::Display;

use anyhow::Result;
use tracing::debug;

use crate::utils::{*, grid::Grid2D};

use super::super::AocDay;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum Rock {
    Round,
    Cube,
    Empty
}

impl Rock {
    fn parse(c: char) -> Self {
        match c {
            '#' => Rock::Cube,
            'O' => Rock::Round,
            _ => Rock::Empty
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

pub struct Day {

}

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
            debug!("Row {} has {} round rocks with {} value each = {}", row, round_count, value, row_value);
        }

        Ok(total_value.into())
    }

    fn run_part2(&mut self, input: &[String]) -> Result<AoCResult> {
        Ok(AoCResult::None)
    }
}

fn tilt_north(grid: &mut Grid2D<Rock>) {
    let rows = grid.height();
    for row_index in 1..rows {

        let row: Vec<_> = grid.get_row(row_index).into_iter().copied().collect();
        for (col_index, _) in row.iter().enumerate().filter(|(_, rock)| **rock == Rock::Round) {

            let destination_row = find_destination(&grid, row_index, col_index);

            let rock_origin = grid.get_mut((col_index, row_index)).unwrap();
            *rock_origin = Rock::Empty;            

            let rock_destination = grid.get_mut((col_index, destination_row)).unwrap();
            *rock_destination = Rock::Round;
        }
    }
}

fn find_destination(grid: &&mut Grid2D<Rock>, current_row: usize, col_index: usize) -> usize {
    for row in (0..current_row).rev() {
        if *grid.get((col_index, row)).unwrap() != Rock::Empty {
            return row+1;
        }
    }

    0
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

        assert_eq!(Point::new(1,2), *found.first().unwrap(), "first");
        assert_eq!(Point::new(1,3), *found.last().unwrap(), "last");
    }
}