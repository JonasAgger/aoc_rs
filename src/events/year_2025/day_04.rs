use anyhow::Result;

use crate::utils::{grid::Grid2D, *};

use super::super::AocDay;

pub struct Day {}

impl Day {
    pub fn new() -> Self {
        Self {}
    }
}

impl AocDay for Day {
    fn run_part1(&mut self, input: &[String]) -> Result<AoCResult> {
        let mut total = 0;
        let grid = Grid2D::parse_char(input, |x| x == '@');

        for point in grid.point_iter() {
            if !grid.get(point).unwrap() {
                continue;
            }

            if get_neighbours_grid_cmp(point, &grid, |&x| x).len() < 4 {
                total += 1;
            }
        }

        Ok(total.into())
    }

    fn run_part2(&mut self, input: &[String]) -> Result<AoCResult> {
        let mut total = 0;
        let mut grid = Grid2D::parse_char(input, |x| x == '@');

        loop {
            let mut current_round_removal = 0;
            for point in grid.point_iter() {
                if !grid.get(point).unwrap() {
                    continue;
                }

                if get_neighbours_grid_cmp(point, &grid, |&x| x).len() < 4 {
                    current_round_removal += 1;
                    grid.set(point, false);
                }
            }

            total += current_round_removal;
            if current_round_removal == 0 {
                break;
            }
        }

        Ok(total.into())
    }
}
