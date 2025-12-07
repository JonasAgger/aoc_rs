use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign},
};

use super::vec2d::Vec2D;

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct IPoint {
    pub x: i64,
    pub y: i64,
}

impl IPoint {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> i64 {
        self.x
    }

    pub fn y(&self) -> i64 {
        self.y
    }

    pub fn add_x(&self, x: i64) -> Option<Self> {
        self.x
            .checked_add(x)
            .map(|new_x| IPoint::new(new_x, self.y))
    }

    pub fn sub_x(&self, x: i64) -> Option<Self> {
        self.x
            .checked_sub(x)
            .map(|new_x| IPoint::new(new_x, self.y))
    }

    pub fn add_y(&self, y: i64) -> Option<Self> {
        self.y
            .checked_add(y)
            .map(|new_y| IPoint::new(self.x, new_y))
    }

    pub fn sub_y(&self, y: i64) -> Option<Self> {
        self.y
            .checked_sub(y)
            .map(|new_y| IPoint::new(self.x, new_y))
    }

    pub fn manhattan_distance(&self, other: &IPoint) -> i64 {
        (self.x.abs_diff(other.x) + self.y.abs_diff(other.y)) as i64
    }
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }

    pub fn add_x(&self, x: usize) -> Option<Self> {
        self.x.checked_add(x).map(|new_x| Point::new(new_x, self.y))
    }

    pub fn sub_x(&self, x: usize) -> Option<Self> {
        self.x.checked_sub(x).map(|new_x| Point::new(new_x, self.y))
    }

    pub fn add_y(&self, y: usize) -> Option<Self> {
        self.y.checked_add(y).map(|new_y| Point::new(self.x, new_y))
    }

    pub fn sub_y(&self, y: usize) -> Option<Self> {
        self.y.checked_sub(y).map(|new_y| Point::new(self.x, new_y))
    }

    pub fn is_neighbour_straight(&self, other: &Point) -> bool {
        (self.x.abs_diff(other.x) + self.y.abs_diff(other.y)) == 1
    }

    pub fn is_neighbour_diag(&self, other: &Point) -> bool {
        self.x.abs_diff(other.x) <= 1 && self.y.abs_diff(other.y) <= 1
    }

    pub fn euclidian_distance_lossy(&self, other: &Point) -> usize {
        let dist = (self.x - other.x).pow(2) + (self.y - other.y).pow(2);
        (dist as f64).sqrt() as usize
    }

    pub fn manhattan_distance(&self, other: &Point) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    pub fn get_diff<P: Into<Point>>(&self, other: P) -> Vec2D {
        let Point { x, y } = other.into();
        let x = self.x as i64 - x as i64;
        let y = self.y as i64 - y as i64;
        Vec2D::new(x, y)
    }

    pub fn checked_add(&self, rhs: Vec2D) -> Option<Self> {
        let mut res = *self;
        res = if rhs.x() < 0 {
            res.sub_x(rhs.x().abs() as usize)?
        } else {
            res.add_x(rhs.x() as usize)?
        };

        if rhs.y() < 0 {
            res.sub_y(rhs.y().abs() as usize)
        } else {
            res.add_y(rhs.y() as usize)
        }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x:{}, y:{})", self.x, self.y)
    }
}

impl Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for IPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x:{}, y:{})", self.x, self.y)
    }
}

impl Debug for IPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl From<Point> for (usize, usize) {
    fn from(e: Point) -> (usize, usize) {
        let Point { x, y } = e;
        (x, y)
    }
}

impl From<&Point> for (usize, usize) {
    fn from(e: &Point) -> (usize, usize) {
        let Point { x, y } = e;
        (*x, *y)
    }
}

impl From<(usize, usize)> for Point {
    fn from(e: (usize, usize)) -> Point {
        Point::new(e.0, e.1)
    }
}

impl From<&(usize, usize)> for Point {
    fn from(e: &(usize, usize)) -> Point {
        Point::new(e.0, e.1)
    }
}

impl Add<Vec2D> for Point {
    type Output = Point;

    fn add(mut self, rhs: Vec2D) -> Self::Output {
        self = if rhs.x() < 0 {
            self.sub_x(rhs.x().abs() as usize).unwrap()
        } else {
            self.add_x(rhs.x() as usize).unwrap()
        };

        if rhs.y() < 0 {
            self.sub_y(rhs.y().abs() as usize).unwrap()
        } else {
            self.add_y(rhs.y() as usize).unwrap()
        }
    }
}

impl AddAssign<Vec2D> for Point {
    fn add_assign(&mut self, rhs: Vec2D) {
        *self = *self + rhs;
    }
}

impl Add<Vec2D> for IPoint {
    type Output = IPoint;

    fn add(self, rhs: Vec2D) -> Self::Output {
        self.add_x(rhs.x()).unwrap().add_y(rhs.y()).unwrap()
    }
}
