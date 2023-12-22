use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
};

use anyhow::Result;
use tracing::debug;

use crate::utils::{grid::Grid2D, *};

use super::super::AocDay;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum Mirror {
    Empty,
    RMirror,
    LMirror,
    HSplitter,
    VSplitter,
}

impl Mirror {
    fn parse(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '/' => Self::RMirror,
            '\\' => Self::LMirror,
            '-' => Self::HSplitter,
            '|' => Self::VSplitter,
            _ => unreachable!(),
        }
    }

    fn next_direction(&self, d: Direction) -> Direction {
        match self {
            Mirror::Empty => d,
            Mirror::RMirror => match d {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
            },
            Mirror::LMirror => match d {
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            },
            Mirror::HSplitter => match d {
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Left,
                Direction::Right => Direction::Right,
            },
            Mirror::VSplitter => match d {
                Direction::Up => Direction::Up,
                Direction::Down => Direction::Down,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Up,
            },
        }
    }
}

impl Display for Mirror {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mirror::Empty => write!(f, "."),
            Mirror::RMirror => write!(f, "/"),
            Mirror::LMirror => write!(f, "\\"),
            Mirror::HSplitter => write!(f, "-"),
            Mirror::VSplitter => write!(f, "|"),
        }
    }
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn inverse(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Beam(Point, Direction);

pub struct Day {}

impl Day {
    pub fn new() -> Self {
        Self {}
    }
}

impl AocDay for Day {
    fn run_part1(&mut self, input: &[String]) -> Result<AoCResult> {
        let mut grid = Grid2D::parse(input, |l| l.chars().map(Mirror::parse).collect());

        let points = solve(Beam(Point::new(0, 0), Direction::Right), &mut grid);

        Ok(points.into())
    }

    fn run_part2(&mut self, input: &[String]) -> Result<AoCResult> {
        let mut grid = Grid2D::parse(input, |l| l.chars().map(Mirror::parse).collect());

        let mut starts = vec![];

        for i in 0..grid.width() {
            starts.push(Beam(Point::new(i, 0), Direction::Down));
            starts.push(Beam(Point::new(i, grid.height() - 1), Direction::Up));
        }

        for i in 0..grid.height() {
            starts.push(Beam(Point::new(0, i), Direction::Right));
            starts.push(Beam(Point::new(grid.width() - 1, i), Direction::Left));
        }

        let points = starts.iter().map(|b| solve(*b, &mut grid)).max().unwrap();

        Ok(points.into())
    }
}

fn solve(start_beam: Beam, grid: &mut Grid2D<Mirror>) -> usize {
    let mut visited = HashSet::new();

    let mut beams = VecDeque::new();

    let beam = start_beam;

    let space = grid.get(beam.0).unwrap();
    let next_direction = space.next_direction(beam.1);
    beams.push_back(Beam(beam.0, next_direction));

    while let Some(beam) = beams.pop_front() {
        if !visited.insert(beam) {
            continue;
        }

        // what happens next

        let next_point = match beam.1 {
            Direction::Up => beam.0.sub_y(1),
            Direction::Down => beam.0.add_y(1),
            Direction::Left => beam.0.sub_x(1),
            Direction::Right => beam.0.add_x(1),
        };

        if let Some(point) = next_point
            && grid.is_within_bounds(point)
        {
            let space = grid.get(point).unwrap();

            let next_direction = space.next_direction(beam.1);

            match *space {
                Mirror::HSplitter | Mirror::VSplitter => {
                    beams.push_back(Beam(point, next_direction.inverse()));
                    beams.push_back(Beam(point, next_direction));
                }
                _ => beams.push_back(Beam(point, next_direction)),
            };
        }
    }

    let points: HashSet<_> = visited.into_iter().map(|b| b.0).collect();
    points.len()
}
