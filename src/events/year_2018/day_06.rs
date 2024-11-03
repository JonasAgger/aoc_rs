use std::{collections::HashSet, fmt::Display, usize};

use anyhow::Result;
use grid::Grid2D;
use variable::{variable, Variable};

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
        let points: Vec<_> = input
            .iter()
            .map(|s| {
                let (x, y) = split_numbers_by(s, ",");
                Point::new(x, y)
            })
            .collect();

        let dims = points.iter().map(|x| x.x.max(x.y)).max().unwrap() + 1;

        let grid = Grid2D::build(dims, dims, |x, y| {
            let source = Point::new(x, y);
            let mut max = usize::MAX;
            let mut owner = Owner::None;

            for (id, p) in points.iter().enumerate() {
                let dist = p.manhattan_distance(&source);

                if dist == 0 {
                    max = dist;
                    owner = Owner::Source(id)
                } else if dist < max {
                    max = dist;
                    owner = Owner::Owner(id);
                } else if dist == max {
                    owner = Owner::None;
                }
            }

            owner
        });

        let infinites: HashSet<usize> = {
            let top = grid.get_row(0);
            let bottom = grid.get_row(dims - 1);

            let left = grid.get_col(0);
            let right = grid.get_col(dims - 1);

            top.into_iter()
                .chain(bottom.into_iter())
                .chain(left.into_iter())
                .chain(right.into_iter())
                .filter_map(|x| match x {
                    Owner::Source(id) => Some(*id),
                    Owner::Owner(id) => Some(*id),
                    Owner::None => None,
                })
                .collect()
        };

        let other: Vec<usize> = (0..points.len())
            .filter(|x| !infinites.contains(x))
            .collect();

        let largest_area = other
            .iter()
            .map(|id| grid.find_all(|x| x == id).len())
            .max()
            .unwrap();

        Ok(largest_area.into())
    }

    fn run_part2(&mut self, input: &[String]) -> Result<AoCResult> {
        const DIST: Variable<usize> = variable(32, 10000);

        let points: Vec<_> = input
            .iter()
            .map(|s| {
                let (x, y) = split_numbers_by(s, ",");
                Point::new(x, y)
            })
            .collect();

        let dims = points.iter().map(|x| x.x.max(x.y)).max().unwrap() + 1;

        let grid = Grid2D::build(dims, dims, |x, y| {
            let source = Point::new(x, y);

            let dist: usize = points.iter().map(|p| p.manhattan_distance(&source)).sum();

            if dist < *DIST {
                1
            } else {
                0
            }
        });

        let count = grid.find_all(|&x| x == 1).len();

        Ok(count.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Owner {
    Source(usize),
    Owner(usize),
    None,
}

impl Display for Owner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Owner::Source(_) => write!(f, "X"),
            Owner::Owner(id) => write!(f, "{}", id),
            Owner::None => write!(f, "."),
        }
    }
}

impl PartialEq<usize> for Owner {
    fn eq(&self, other: &usize) -> bool {
        match self {
            Self::Source(id) => id == other,
            Self::Owner(id) => id == other,
            _ => false,
        }
    }
}
