use core::panic;
use std::collections::{HashSet, VecDeque};

use anyhow::Result;

use crate::utils::{grid::Grid2D, point::IPoint, *};

use super::super::AocDay;

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn parse(c: &str) -> Self {
        match c {
            "U" | "3" => Direction::Up,
            "D" | "1" => Direction::Down,
            "L" | "2" => Direction::Left,
            "R" | "0" => Direction::Right,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct DigPlan {
    direction: Direction,
    count: i64,
}

impl DigPlan {
    fn parse(line: &str) -> Self {
        let mut it = line.split_ascii_whitespace();
        Self {
            direction: Direction::parse(it.next().unwrap()),
            count: it.next().unwrap().parse().unwrap(),
        }
    }

    fn parse2(line: &str) -> Self {
        let hex = line.split_ascii_whitespace().last().unwrap();
        let hex = &hex[2..hex.len() - 1];
        Self {
            direction: Direction::parse(&hex[5..]),
            count: i64::from_str_radix(&hex[..5], 16).unwrap(),
        }
    }

    fn dig(&self, localtion: IPoint) -> IPoint {
        let new = match self.direction {
            Direction::Up => localtion.sub_y(self.count),
            Direction::Down => localtion.add_y(self.count),
            Direction::Left => localtion.sub_x(self.count),
            Direction::Right => localtion.add_x(self.count),
        };

        new.unwrap()
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
        let plans: Vec<_> = input.iter().map(|line| DigPlan::parse(line)).collect();

        let mut current_location = IPoint::new(0, 0);

        let mut visited = vec![current_location];

        for dig_plan in plans {
            current_location = dig_plan.dig(current_location);

            visited.push(current_location);
        }

        let fill_cnt = fill(&visited);

        Ok(fill_cnt.into())
    }

    fn run_part2(&mut self, input: &[String]) -> Result<AoCResult> {
        let plans: Vec<_> = input.iter().map(|line| DigPlan::parse2(line)).collect();

        let mut current_location = IPoint::new(0, 0);

        let mut visited = vec![current_location];

        for dig_plan in plans {
            current_location = dig_plan.dig(current_location);

            visited.push(current_location);
        }

        let fill_cnt = picks_theorem_and_shoelace(&visited);

        Ok(fill_cnt.into())
    }
}

fn perimiter(points: &[IPoint]) -> i64 {
    points
        .iter()
        .map_windows(|[&a, &b]| b.manhattan_distance(&a))
        .sum()
}

fn shoelace(points: &[IPoint]) -> i64 {
    // We can calculate the area by making a lot of triangles, and then just adding them up
    let sum: i64 = points
        .iter()
        .map_windows(|[&a, &b]| a.x() * b.y() - b.x() * a.y())
        .sum();
    sum.abs() / 2
}

fn picks_theorem_and_shoelace(points: &[IPoint]) -> i64 {
    let perimiter = perimiter(points);
    let area = shoelace(points);

    area + perimiter / 2 + 1
}

fn fill(points: &[IPoint]) -> usize {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut x_offset = 0;
    let mut y_offset = 0;
    for p in points.iter() {
        max_x = max_x.max(p.x());
        max_y = max_y.max(p.y());
        x_offset = x_offset.min(p.x());
        y_offset = y_offset.min(p.y());
    }

    x_offset = x_offset.abs();
    y_offset = y_offset.abs();

    let mut grid = Grid2D::new(
        (x_offset + max_x + 1) as usize,
        (y_offset + max_y + 1) as usize,
        '.',
    );

    for i in 0..points.len() - 1 {
        let p1 = points[i];
        let p2 = points[i + 1];

        if p1.x() == p2.x() {
            let range = match p1.y() < p2.y() {
                true => p1.y()..=p2.y(),
                false => p2.y()..=p1.y(),
            };
            for y in range {
                let p = Point::new((p1.x() + x_offset) as usize, (y + y_offset) as usize);
                *grid.get_mut(p).unwrap() = '#';
            }
        } else {
            let range = match p1.x() < p2.x() {
                true => p1.x()..=p2.x(),
                false => p2.x()..=p1.x(),
            };
            for x in range {
                let p = Point::new((x + x_offset) as usize, (p1.y() + y_offset) as usize);
                *grid.get_mut(p).unwrap() = '#';
            }
        }
    }

    let start_flood_fill = Point::new(grid.width() / 2, grid.height() / 2);

    let mut visited = HashSet::new();
    let mut flood_fill = VecDeque::new();

    flood_fill.push_back(start_flood_fill);
    visited.insert(start_flood_fill);

    while let Some(p) = flood_fill.pop_front() {
        *grid.get_mut(p).unwrap() = '#';
        let neighbours = Utils::get_neighbours_grid_cmp(p, &grid, |&c| c == '.');
        // dbg!(&neighbours);
        for n in neighbours {
            if visited.insert(n) {
                flood_fill.push_back(n);
            }
        }
    }

    grid.find_all(|&c| c == '#').len()
}
