use std::fmt::{Debug, Display};

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
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

    // pub fn is_neighbour_straight(&self, other: &Point) -> bool {
    //     self.x.abs_diff(other.x) == 1 && self.y.abs_diff(other.y) == 1
    // }

    pub fn is_neighbour_diag(&self, other: &Point) -> bool {
        self.x.abs_diff(other.x) <= 1 && self.y.abs_diff(other.y) <= 1
    }

    // pub fn euclidian_distance(&self, other: &Point) -> f64 {
    //     let dist = (self.x - other.x).pow(2) + (self.y - other.y).pow(2);
    //     (dist as f64).sqrt()
    // }

    pub fn manhattan_distance(&self, other: &Point) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
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
