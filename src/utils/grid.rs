use std::{
    fmt::Display,
    hash::{DefaultHasher, Hasher},
};

use super::Point;

#[derive(Debug, Clone)]
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

    pub fn parse_char<F: Fn(char) -> T>(input: &[String], parser: F) -> Self {
        let mut backing_vec = vec![];
        let mut row_width = 0;

        for line in input {
            row_width = line.len();

            let parsed: Vec<_> = line.chars().map(&parser).collect();
            backing_vec.extend_from_slice(parsed.as_slice());
        }

        Self {
            backing_vec,
            row_width,
        }
    }

    pub fn parse_char_it<'a, IT: Iterator<Item = &'a String>, F: Fn(char) -> T>(
        input: IT,
        parser: F,
    ) -> Self {
        let mut backing_vec = vec![];
        let mut row_width = 0;

        for line in input {
            row_width = line.len();

            let parsed: Vec<_> = line.chars().map(&parser).collect();
            backing_vec.extend_from_slice(parsed.as_slice());
        }

        Self {
            backing_vec,
            row_width,
        }
    }

    pub fn build<F: Fn(usize, usize) -> T>(row_width: usize, rows: usize, factory: F) -> Self {
        let mut backing_vec = Vec::with_capacity(row_width * rows);

        for row in 0..rows {
            for col in 0..row_width {
                backing_vec.push(factory(col, row));
            }
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

    pub fn from_raw(data: Vec<T>, row_width: usize) -> Self {
        Self {
            backing_vec: data,
            row_width,
        }
    }

    pub fn width(&self) -> usize {
        self.row_width
    }

    pub fn height(&self) -> usize {
        self.backing_vec.len() / self.row_width
    }

    pub fn set<P: Into<Point>>(&mut self, p: P, item: T) {
        let Point { x, y } = p.into();
        match self.is_within_bounds((x, y)) {
            true => {
                let index = y * self.row_width + x;
                self.backing_vec[index] = item;
            }
            false => panic!("point: {:?} was not inside grid bounds", x),
        }
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

    pub fn find_all<F: Fn(&T) -> bool>(&self, f: F) -> Vec<Point> {
        self.backing_vec
            .iter()
            .enumerate()
            .filter(|(_, item)| f(*item))
            .map(|(index, _)| {
                let y = index / self.row_width;
                let x = index % self.row_width;
                Point::new(x, y)
            })
            .collect()
    }

    pub fn is_within_bounds<P: Into<Point>>(&self, p: P) -> bool {
        let Point { x, y } = p.into();
        x < self.width() && y < self.height()
    }

    pub fn get_row(&self, row: usize) -> impl IntoIterator<Item = &T> {
        let offset = row * self.row_width;
        self.backing_vec[offset..offset + self.row_width].iter()
    }

    pub fn get_col(&self, col: usize) -> impl IntoIterator<Item = &T> {
        self.backing_vec[col..].iter().step_by(self.row_width)
    }

    pub fn swap<P: Into<Point>>(&mut self, src: P, dest: P) {
        let src = src.into();
        let dest = dest.into();
        if self.is_within_bounds(src) && self.is_within_bounds(dest) {
            let src_index = src.y() * self.row_width + src.x();
            let dest_index = dest.y() * self.row_width + dest.x();
            self.backing_vec.swap(src_index, dest_index);
        } else {
            panic!("Tried to swap invalid indicies {} -- {}", src, dest);
        }
    }

    pub fn point_iter(&self) -> PointIter {
        PointIter {
            len: self.backing_vec.len(),
            width: self.row_width,
            index: 0,
        }
    }
}

pub struct PointIter {
    len: usize,
    width: usize,
    index: usize,
}

impl Iterator for PointIter {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.len {
            return None;
        }

        let y = self.index / self.width;
        let x = self.index % self.width;
        self.index += 1;
        Some(Point::new(x, y))
    }
}

impl<T: Clone + Display + std::hash::Hash> Grid2D<T> {
    pub fn hash_state(&self) -> u64 {
        use std::hash::Hash;
        let mut hasher = DefaultHasher::new();
        self.backing_vec.hash(&mut hasher);
        hasher.finish()
    }
}

impl<T: Clone + Display> Display for Grid2D<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for i in (0..self.backing_vec.len()).step_by(self.row_width) {
            for j in i..i + self.row_width {
                write!(f, "{}", self.backing_vec[j])?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
