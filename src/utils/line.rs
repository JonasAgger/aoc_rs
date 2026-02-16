use std::fmt::Display;

use crate::utils::{Point, grid::Grid2D, point::IPoint};

#[derive(Debug, Clone, Copy)]
pub struct Line {
    p1: IPoint,
    p2: IPoint,
}

impl Line {
    pub const fn empty() -> Self {
        Self::new(IPoint { x: 0, y: 0 }, IPoint { x: 0, y: 0 })
    }

    pub const fn new(p1: IPoint, p2: IPoint) -> Self {
        Self { p1, p2 }
    }

    pub fn intersects(&self, other: &Line) -> bool {
        if self == other {
            return false;
        }

        let oa = oritent(other.p1, other.p2, self.p1);
        let ob = oritent(other.p1, other.p2, self.p2);
        let oc = oritent(self.p1, self.p2, other.p1);
        let od = oritent(self.p1, self.p2, other.p2);

        oa * ob < 0 && oc * od < 0
    }

    pub fn has_x_slope(&self) -> bool {
        self.p2.x() != self.p1.x()
    }

    pub fn has_y_slope(&self) -> bool {
        self.p2.x() != self.p1.x()
    }

    pub fn same_slope(&self, other: &Line) -> bool {
        self.has_x_slope() && other.has_x_slope()
    }

    pub fn points(&self) -> [&IPoint; 2] {
        [&self.p1, &self.p2]
    }

    pub fn widen(self, amount: i64) -> Line {
        if self.has_x_slope() {
            let min = self.p1.x.min(self.p2.x) - amount;
            let max = self.p1.x.max(self.p2.x) + amount;

            Line::new(IPoint::new(min, self.p1.y), IPoint::new(max, self.p1.y))
        } else {
            let min = self.p1.y.min(self.p2.y) - amount;
            let max = self.p1.y.max(self.p2.y) + amount;

            Line::new(IPoint::new(self.p1.x, min), IPoint::new(self.p1.x, max))
        }
    }
}

impl PartialEq for Line {
    fn eq(&self, other: &Self) -> bool {
        (self.p1 == other.p1 || self.p1 == other.p2) && (self.p2 == other.p2 || self.p2 == other.p1)
    }
}

fn cross(p1: IPoint, p2: IPoint) -> i64 {
    p1.x * p2.y - p1.y * p2.x
}

fn oritent(p1: IPoint, p2: IPoint, p3: IPoint) -> i64 {
    cross(p2 - p1, p3 - p1)
}

pub fn draw<T: Clone + Default + Display>(line: &Line, grid: &mut Grid2D<T>, item: T) {
    if line.has_x_slope() {
        let start = line.p1.x.min(line.p2.x);
        let end = line.p1.x.max(line.p2.x);
        for idx in start..=end {
            grid.set(Point::new(idx as usize, line.p1.y as usize), item.clone());
        }
    } else {
        let start = line.p1.y.min(line.p2.y);
        let end = line.p1.y.max(line.p2.y);
        for idx in start..end {
            grid.set(Point::new(line.p1.x as usize, idx as usize), item.clone());
        }
    }
}
