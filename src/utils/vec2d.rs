use std::{fmt::Display, ops::Add};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Vec2D {
    x: i64,
    y: i64,
}

impl Vec2D {
    pub const UP: Vec2D = Vec2D::new(0, -1);
    pub const DOWN: Vec2D = Vec2D::new(0, 1);
    pub const LEFT: Vec2D = Vec2D::new(-1, 0);
    pub const RIGHT: Vec2D = Vec2D::new(1, 0);

    pub const fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub const fn x(&self) -> i64 {
        self.x
    }

    pub const fn y(&self) -> i64 {
        self.y
    }

    pub fn magnitude(&self) -> usize {
        (self.x.abs() + self.y.abs()) as usize
    }
}

impl Add for Vec2D {
    type Output = Vec2D;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl From<(i64, i64)> for Vec2D {
    fn from(e: (i64, i64)) -> Vec2D {
        Vec2D::new(e.0, e.1)
    }
}

impl From<&(i64, i64)> for Vec2D {
    fn from(e: &(i64, i64)) -> Vec2D {
        Vec2D::new(e.0, e.1)
    }
}

impl Display for Vec2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "|{}, {}|", self.x, self.y)
    }
}
