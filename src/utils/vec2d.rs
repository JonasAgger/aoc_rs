#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Vec2D {
    x: i64,
    y: i64,
}

impl Vec2D {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> i64 {
        self.x
    }

    pub fn y(&self) -> i64 {
        self.y
    }

    pub fn magnitude(&self) -> usize {
        (self.x.abs() + self.y.abs()) as usize
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
