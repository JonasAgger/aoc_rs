use std::{collections::VecDeque, fmt::Display, ops::AddAssign};

use anyhow::Result;

use crate::utils::{grid::Grid2D, vec2d::Vec2D, *};

use super::super::AocDay;

pub struct Day {}

impl Day {
    pub fn new() -> Self {
        Self {}
    }
}

impl AocDay for Day {
    fn run_part1(&mut self, input: &[String]) -> Result<AoCResult> {
        let mut grid = Grid2D::parse_char(&input[1..], |c| {
            if c == '^' {
                Tachyon::Splitter
            } else {
                Tachyon::Empty
            }
        });

        // Set start beam
        let start_idx = input[0].find('S').unwrap();

        let mut beam_queue = VecDeque::new();
        beam_queue.push_back(Point::new(start_idx, 0));

        let mut split_count = 0;

        while let Some(mut beam_position) = beam_queue.pop_front() {
            beam_position += Vec2D::DOWN;

            // check if split
            if grid.get(beam_position) == Some(&Tachyon::Splitter) {
                let left = beam_position.sub_x(1);
                let right = beam_position.add_x(1);

                let points_to_add: Vec<_> = [left, right]
                    .into_iter()
                    .filter_map(|x| x)
                    .filter(|&p| grid.get(p).is_some_and(|x| x.is_empty()))
                    .collect();

                for p in points_to_add {
                    grid.set(beam_position, Tachyon::Beam(0));
                    beam_queue.push_back(p);
                }
                split_count += 1;
            } else if grid.get(beam_position).is_some_and(|x| x.is_empty()) {
                grid.set(beam_position, Tachyon::Beam(0));
                beam_queue.push_back(beam_position);
            }
        }

        Ok(split_count.into())
    }

    fn run_part2(&mut self, input: &[String]) -> Result<AoCResult> {
        let mut grid = Grid2D::parse_char(&input[1..], |c| {
            if c == '^' {
                Tachyon::Splitter
            } else {
                Tachyon::Empty
            }
        });

        // Set start beam
        let start_idx = input[0].find('S').unwrap();
        grid.set(Point::new(start_idx, 0), Tachyon::Beam(1));

        let mut beam_queue = VecDeque::new();
        beam_queue.push_back(Point::new(start_idx, 0));

        while let Some(mut beam_position) = beam_queue.pop_front() {
            let current = *grid.get(beam_position).unwrap();
            beam_position += Vec2D::DOWN;

            // check if split
            if grid.get(beam_position) == Some(&Tachyon::Splitter) {
                let left = beam_position.sub_x(1);
                let right = beam_position.add_x(1);

                let points_to_add: Vec<_> = [left, right]
                    .into_iter()
                    .filter_map(|x| x)
                    .filter(|&p| grid.is_within_bounds(p))
                    .collect();

                for p in points_to_add {
                    let entry = grid.get_mut(p).unwrap();
                    match entry {
                        Tachyon::Empty => {
                            *entry = current;
                            beam_queue.push_back(p);
                        }
                        Tachyon::Beam(count) => count.add_assign(current.count()),
                        Tachyon::Splitter => todo!(),
                    }
                }
            } else if grid.is_within_bounds(beam_position) {
                let entry = grid.get_mut(beam_position).unwrap();
                match entry {
                    Tachyon::Empty => {
                        *entry = current;
                        beam_queue.push_back(beam_position);
                    }
                    Tachyon::Beam(count) => count.add_assign(current.count()),
                    Tachyon::Splitter => todo!(),
                };
            }
        }

        let count: usize = grid
            .get_row(grid.height() - 1)
            .into_iter()
            .map(|x| x.count())
            .sum();

        Ok(count.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tachyon {
    Empty,
    Beam(usize),
    Splitter,
}

impl Tachyon {
    fn count(&self) -> usize {
        match self {
            Tachyon::Empty => 0,
            Tachyon::Beam(c) => *c,
            Tachyon::Splitter => 0,
        }
    }

    fn is_empty(&self) -> bool {
        matches!(self, Tachyon::Empty)
    }
}

impl Display for Tachyon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tachyon::Empty => write!(f, "."),
            Tachyon::Beam(c) => write!(f, "{:x}", c),
            Tachyon::Splitter => write!(f, "^"),
        }
    }
}
