use std::collections::HashSet;

use anyhow::Result;
use grid::Grid2D;
use slice_utils::Single;

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
        const DIMENSION: usize = 1000;
        let rectangles: Vec<_> = input.iter().map(|s| Rectangle::from(s.as_str())).collect();

        let mut grid = Grid2D::new(DIMENSION, DIMENSION, 0);

        for rectangle in rectangles {
            let Point { x, y } = rectangle.point;

            let y1 = DIMENSION - y;
            let y2 = y1 - rectangle.height;

            for y in y2..y1 {
                for x in x..(x + rectangle.width) {
                    let p = Point::new(x, y);
                    let val = grid.get_mut(p).unwrap();
                    *val = *val + 1;
                }
            }
        }

        let sum = grid.find_all(|&c| c > 1);
        Ok(sum.len().into())
    }

    fn run_part2(&mut self, input: &[String]) -> Result<AoCResult> {
        const DIMENSION: usize = 1000;
        let rectangles: Vec<_> = input.iter().map(|s| Rectangle::from(s.as_str())).collect();
        let mut ids: HashSet<usize> = rectangles.iter().map(|x| x.id).collect();

        let mut grid = Grid2D::new(DIMENSION, DIMENSION, 0);

        for rectangle in rectangles {
            let Point { x, y } = rectangle.point;

            let y1 = DIMENSION - y;
            let y2 = y1 - rectangle.height;

            for y in y2..y1 {
                for x in x..(x + rectangle.width) {
                    let p = Point::new(x, y);
                    let val = grid.get_mut(p).unwrap();
                    if *val != 0 {
                        ids.remove(val);
                        ids.remove(&rectangle.id);
                    } else {
                        *val = rectangle.id;
                    }
                }
            }
        }

        Ok(ids.into_iter().single().into())
    }
}

struct Rectangle {
    id: usize,
    point: Point,
    width: usize,
    height: usize,
}

impl From<&str> for Rectangle {
    fn from(value: &str) -> Self {
        let items: Vec<_> = value.split_whitespace().collect();

        let (x, y) = split_numbers_by(items[2], ',');
        let (w, h) = split_numbers_by(items.last().unwrap(), 'x');

        Rectangle {
            id: items[0].number(),
            point: Point::new(x, y),
            width: w,
            height: h,
        }
    }
}
