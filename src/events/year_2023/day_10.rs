use std::{collections::HashSet, fmt::Display, ops::Add};

use anyhow::Result;
use tracing::debug;

use crate::utils::{grid::Grid2D, *};

use super::super::AocDay;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pipe {
    Vert,
    Hori,
    NE_Bend,
    NW_Bend,
    SW_Bend,
    SE_Bend,
    Ground,
    Start,
}

impl Pipe {
    pub fn parse(c: char) -> Self {
        match c {
            '|' => Pipe::Vert,
            '-' => Pipe::Hori,
            'L' => Pipe::NE_Bend,
            'J' => Pipe::NW_Bend,
            '7' => Pipe::SW_Bend,
            'F' => Pipe::SE_Bend,
            '.' => Pipe::Ground,
            'S' => Pipe::Start,
            _ => panic!("Found unexpected char while parsing Pipe"),
        }
    }
    // returns list of (x,y)
    fn get_connecting(&self) -> Vec<(i32, i32)> {
        match self {
            Pipe::Hori => vec![(-1, 0), (1, 0)],
            Pipe::Vert => vec![(0, -1), (0, 1)],

            Pipe::NE_Bend => vec![(1, 0), (0, -1)],
            Pipe::NW_Bend => vec![(-1, 0), (0, -1)],

            Pipe::SW_Bend => vec![(-1, 0), (0, 1)],
            Pipe::SE_Bend => vec![(1, 0), (0, 1)],

            Pipe::Ground => vec![],
            Pipe::Start => vec![],
        }
    }

    pub fn get_connecting_points(&self, localtion: &Point) -> Vec<Point> {
        let mut points = vec![];
        for (x_diff, y_diff) in self.get_connecting() {
            let end_x = match x_diff {
                -1 => localtion.x.checked_sub(1),
                1 => localtion.x.checked_add(1),
                _ => Some(localtion.x),
            };
            let end_y = match y_diff {
                -1 => localtion.y.checked_sub(1),
                1 => localtion.y.checked_add(1),
                _ => Some(localtion.y),
            };
            debug!("{} - {:?}{:?}", self, end_x, end_y);
            match (end_x, end_y) {
                (Some(x), Some(y)) => points.push((x, y).into()),
                (_, _) => continue,
            };
        }
        points
    }

    pub fn can_connect(&self, localtion: &Point, destination: &Point) -> bool {
        self.get_connecting_points(localtion)
            .iter()
            .any(|p| p == destination)
    }

    pub fn get_next(&self, localtion: &Point, previous: &Point) -> Point {
        self.get_connecting_points(localtion)
            .iter()
            .filter(|&p| p != previous)
            .next()
            .unwrap()
            .clone()
    }
}

impl Display for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pipe::Vert => write!(f, "|"),
            Pipe::Hori => write!(f, "-"),
            Pipe::NE_Bend => write!(f, "L"),
            Pipe::NW_Bend => write!(f, "J"),
            Pipe::SW_Bend => write!(f, "7"),
            Pipe::SE_Bend => write!(f, "F"),
            Pipe::Ground => write!(f, "."),
            Pipe::Start => write!(f, "S"),
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
        let grid = Grid2D::parse(input, |s| s.chars().map(|c| Pipe::parse(c)).collect());
        let start = grid.find(|x| matches!(x, &Pipe::Start)).unwrap();
        let pipe_path = get_pipe_path(&start, &grid);

        let furtest_away_steps = pipe_path.len() / 2 + 1;
        Ok(furtest_away_steps.into())
    }

    fn run_part2(&mut self, input: &[String]) -> Result<AoCResult> {
        let grid = Grid2D::parse(input, |s| s.chars().map(|c| Pipe::parse(c)).collect());
        let start = grid.find(|x| matches!(x, &Pipe::Start)).unwrap();
        let pipe_path = get_pipe_path(&start, &grid);

        Ok(AoCResult::None)
    }
}

fn get_pipe_path(start: &Point, grid: &Grid2D<Pipe>) -> Vec<Point> {
    let start = *start;

    // lets get all neighbours that are not ground
    let connecting_neighbours =
        Utils::get_neighbours_grid_cmp(start, &grid, |p| !matches!(p, &Pipe::Ground));
    let neighbouring_pipes: Vec<_> = connecting_neighbours
        .iter()
        .map(|&p| grid.get(p).unwrap())
        .collect();
    let connectors: Vec<_> = connecting_neighbours
        .iter()
        .zip(neighbouring_pipes)
        .filter(|(point, pipe)| pipe.can_connect(*point, &start))
        .map(|(point, _)| point)
        .collect();

    debug!("{:?}", connectors);

    let mut prev = start;
    let mut current = **connectors.first().unwrap();
    let mut visited = Vec::new();

    while current != start {
        let pipe = grid.get(current).unwrap();
        let next = pipe.get_next(&current, &prev);
        visited.push(current.clone());
        prev = current;
        current = next;
    }

    visited
}

fn vizualize(visited: &Vec<Point>, grid: Grid2D<char>) {
    let mut cost_grid = Grid2D::new(grid.width(), grid.height(), '.');
    for point in visited {
        let val = cost_grid.get_mut(*point).unwrap();
        *val = '#';
    }
    println!("{}", cost_grid);
}

fn vizualize_costs(visited: &Vec<Point>, grid: Grid2D<char>) {
    let mut cost_grid = Grid2D::new(grid.width(), grid.height(), '.');
    let mut step = 1;
    for i in 0..visited.len() / 2 {
        let head = visited[i];
        let tail = visited[visited.len() - 1 - i];

        let val = cost_grid.get_mut(head).unwrap();
        *val = ('0' as u8 + step as u8) as char;
        let val = cost_grid.get_mut(tail).unwrap();
        *val = ('0' as u8 + step as u8) as char;

        step += 1;
    }

    if visited.len() % 2 == 1 {
        let missing = visited.len() / 2;
        dbg!(missing);
        let missing = visited[missing];
        let val = cost_grid.get_mut(missing).unwrap();
        *val = ('0' as u8 + step as u8) as char;
    }

    println!("{}", cost_grid);
}
