pub mod aoc_result;
pub mod grid;
pub mod math_utils;
pub mod point;
pub mod slice_utils;

use std::fmt::Display;

pub use aoc_result::AoCResult;
pub use point::Point;

use self::grid::Grid2D;

pub struct Utils;

impl Utils {
    pub fn has_neighbour<P: Into<Point>, T: PartialEq>(p: P, map: &Vec<Vec<T>>, val: &T) -> bool {
        get_neighbours(p.into(), map[0].len(), map.len()).any(|(x, y)| map[y][x].eq(val))
    }

    pub fn has_any_neighbour<P: Into<Point>, T: PartialEq>(
        p: P,
        map: &Vec<Vec<T>>,
        vals: &[T],
    ) -> bool {
        get_neighbours(p.into(), map[0].len(), map.len()).any(|(x, y)| {
            let cmp = &map[y][x];
            vals.iter().any(|v| v.eq(cmp))
        })
    }

    pub fn has_cmp_neighbour<P: Into<Point>, T: PartialEq, F: Fn(&T) -> bool>(
        p: P,
        map: &Vec<Vec<T>>,
        cmp: F,
    ) -> bool {
        get_neighbours(p.into(), map[0].len(), map.len()).any(|(x, y)| cmp(&map[y][x]))
    }

    pub fn get_neighbours_cmp<P: Into<Point>, T: PartialEq, F: Fn(&T) -> bool>(
        p: P,
        map: &Vec<Vec<T>>,
        cmp: F,
    ) -> Vec<Point> {
        get_neighbours(p.into(), map[0].len(), map.len())
            .filter(|&(x, y)| cmp(&map[y][x]))
            .map(|t| t.into())
            .collect()
    }

    pub fn get_neighbours_grid_cmp<
        P: Into<Point>,
        T: PartialEq + Clone + Display,
        F: Fn(&T) -> bool,
    >(
        p: P,
        map: &Grid2D<T>,
        cmp: F,
    ) -> Vec<Point> {
        get_neighbours(p.into(), map.width(), map.height())
            .filter(|&(x, y)| cmp(map.get((x, y)).unwrap()))
            .map(|t| t.into())
            .collect()
    }
}

fn get_neighbours(
    p: Point,
    x_bounds: usize,
    y_bounds: usize,
) -> impl Iterator<Item = (usize, usize)> {
    let x_range = if p.x() > 0 {
        p.x() - 1..p.x() + 2
    } else {
        0..2
    };
    let y_range = if p.y() > 0 {
        p.y() - 1..p.y() + 2
    } else {
        0..2
    };
    x_range
        .flat_map(move |x| y_range.clone().map(move |y| Point::new(x, y)))
        .filter(move |&pp| p != pp && pp.x() < x_bounds && pp.y() < y_bounds)
        .map(|p| p.into())
}
