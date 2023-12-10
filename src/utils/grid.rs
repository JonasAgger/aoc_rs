use std::fmt::Display;

use super::Point;

pub struct Grid2D<T: Clone + Display> {
    backing_vec: Vec<T>,
    row_width: usize,
}

impl<T: Clone + Display> Grid2D<T> {
    pub fn parse<F: Fn(&String) -> Vec<T>>(input: &[String], parser: F) -> Self {
        let mut backing_vec = vec![];
        let mut row_width = 0;

        for line in input {
            let parsed = parser(line);
            row_width = parsed.len();
            backing_vec.extend_from_slice(parsed.as_slice());
        }

        Self {
            backing_vec,
            row_width,
        }
    }

    pub fn new(row_width: usize, rows: usize, default: T) -> Self {
        let backing_vec = vec![default; row_width * rows];

        Self {
            backing_vec,
            row_width,
        }
    }

    pub fn width(&self) -> usize {
        self.row_width
    }

    pub fn height(&self) -> usize {
        self.backing_vec.len() / self.row_width
    }

    pub fn get<P: Into<Point>>(&self, p: P) -> Option<&T> {
        let Point { x, y } = p.into();
        match self.is_within_bounds((x, y)) {
            true => {
                let index = y * self.row_width + x;
                Some(&self.backing_vec[index])
            }
            false => None,
        }
    }

    pub fn get_mut<P: Into<Point>>(&mut self, p: P) -> Option<&mut T> {
        let Point { x, y } = p.into();
        match self.is_within_bounds((x, y)) {
            true => {
                let index = y * self.row_width + x;
                Some(&mut self.backing_vec[index])
            }
            false => None,
        }
    }

    pub fn find<F: Fn(&T) -> bool>(&self, f: F) -> Option<Point> {
        for (index, item) in self.backing_vec.iter().enumerate() {
            if f(item) {
                let y = index / self.row_width;
                let x = index % self.row_width;
                return Some(Point::new(x, y));
            }
        }

        None
    }

    pub fn is_within_bounds<P: Into<Point>>(&self, p: P) -> bool {
        let Point { x, y } = p.into();
        x <= self.width() && y <= self.height()
    }
}

impl<T: Clone + Display> Display for Grid2D<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in (0..self.backing_vec.len()).step_by(self.row_width) {
            for j in i..i + self.row_width {
                write!(f, "{}", self.backing_vec[j])?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
