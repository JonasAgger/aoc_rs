pub mod aoc_result;
pub mod grid;
pub mod math_utils;
pub mod point;
pub mod slice_utils;
pub mod variable;
pub mod vec2d;

use std::{
    fmt::{Debug, Display},
    str::{pattern::Pattern, FromStr},
};

pub use aoc_result::AoCResult;
pub use point::Point;

use self::grid::Grid2D;

pub trait StrNumber {
    fn number<T: FromStr<Err = U>, U: Debug>(&self) -> T;
    fn number_or<T: FromStr<Err = U>, U: Debug>(&self, default: T) -> T;
    fn number_in_prefixed<T: FromStr<Err = U>, U: Debug>(&self, prefix: &str) -> T;
}

impl<S: AsRef<str>> StrNumber for S {
    fn number<T: FromStr<Err = U>, U: Debug>(&self) -> T {
        let s = self.as_ref();
        get_number(s)
    }

    fn number_in_prefixed<T: FromStr<Err = U>, U: Debug>(&self, prefix: &str) -> T {
        let s = self.as_ref();

        let index = s.find(prefix).unwrap();

        let c1: String = s
            .chars()
            .skip(index - 1)
            .skip_while(|c| !char::is_ascii_digit(c))
            .take_while(char::is_ascii_digit)
            .collect();
        c1.parse().expect("cannot get number with prefix")
    }

    fn number_or<T: FromStr<Err = U>, U: Debug>(&self, default: T) -> T {
        let s = self.as_ref();
        let c1: String = s
            .chars()
            .skip_while(|c| !char::is_ascii_digit(c))
            .take_while(char::is_ascii_digit)
            .collect();
        c1.parse().unwrap_or(default)
    }
}

pub fn get_number<T: FromStr<Err = U>, U: Debug>(s: &str) -> T {
    let c1: String = s
        .chars()
        .skip_while(|c| !char::is_ascii_digit(c))
        .take_while(char::is_ascii_digit)
        .collect();
    c1.parse().expect("cannot get number")
}

pub fn split_numbers_by<T: FromStr<Err = U>, U: Debug, P: Pattern + Debug + Copy>(
    s: &str,
    pattern: P,
) -> (T, T) {
    let Some((p1, p2)) = s.split_once(pattern) else {
        panic!(
            "Cannot split numbers by: pattern: {:?}\nstr: {}",
            pattern, s
        );
    };

    (get_number(p1), get_number(p2))
}

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

pub fn get_neighbours_straight<P: Into<Point>, T: PartialEq + Clone + Display>(
    p: P,
    map: &Grid2D<T>,
) -> Vec<Point> {
    let point = p.into();
    get_neighbours(point, map.width(), map.height())
        .map(|p| Point::from(p))
        .filter(|p| point.is_neighbour_straight(p))
        .collect()
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
