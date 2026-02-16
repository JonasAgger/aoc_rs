use anyhow::Result;

use crate::utils::{line::Line, point::IPoint, *};

use super::super::AocDay;

pub struct Day {}

impl Day {
    pub fn new() -> Self {
        Self {}
    }
}

impl AocDay for Day {
    fn run_part1(&mut self, input: &[String]) -> Result<AoCResult> {
        let points: Vec<Point> = input
            .into_iter()
            .map(|x| split_numbers_by(&x, ',').into())
            .collect();

        let mut max_rect = usize::MIN;
        for idx in 0..points.len() {
            for i in (idx + 1)..points.len() {
                let manhattan_dist = points[idx].area(&points[i]);
                max_rect = max_rect.max(manhattan_dist);
            }
        }

        Ok(max_rect.into())
    }

    fn run_part2(&mut self, input: &[String]) -> Result<AoCResult> {
        let points: Vec<IPoint> = input
            .into_iter()
            .map(|x| split_numbers_by(&x, ',').into())
            .collect();

        let lines: Vec<_> = generate_walls(&points);

        let mut max_rect = u64::MIN;

        for idx in 0..points.len() {
            let start = idx + 2;
            let end = match idx {
                0 => points.len() - 1,
                _ => points.len(),
            };

            for i in start..end {
                let p1 = points[idx];
                let p2 = points[i];
                let Some(rect) = generate_rect(p1, p2) else {
                    continue;
                };

                let intersected = rect
                    .iter()
                    .any(|line| lines.iter().any(|x| x.intersects(line)));

                if !intersected {
                    let manhattan_dist = points[idx].area(&points[i]);
                    max_rect = max_rect.max(manhattan_dist);
                }
            }
        }

        Ok(max_rect.into())
    }
}

fn generate_walls(points: &[IPoint]) -> Vec<Wall> {
    points
        .iter()
        .enumerate()
        .map(|(idx, &p1)| Wall::new(Line::new(p1, points[(idx + 1) % points.len()])))
        .collect()
}

fn generate_rect(p1: IPoint, p2: IPoint) -> Option<[Line; 4]> {
    if p1.x == p2.x || p1.y == p2.y {
        return None;
    }

    let lines = [
        Line::new(p1, IPoint::new(p1.x, p2.y)),
        Line::new(p1, IPoint::new(p2.x, p1.y)),
        Line::new(p2, IPoint::new(p1.x, p2.y)),
        Line::new(p2, IPoint::new(p2.x, p1.y)),
    ];

    Some(lines)
}

#[derive(Debug)]
struct Wall {
    line: Line,
}

impl Wall {
    fn new(line: Line) -> Wall {
        Self { line }
    }
}

impl Wall {
    fn intersects(&self, other: &Line) -> bool {
        if self.line.same_slope(other) {
            return false;
            // return self.same_slope_intersection(other);
        } else {
            self.line.widen(1).intersects(other)
        }
    }
    fn same_slope_intersection(&self, other: &Line) -> bool {
        if &self.line == other {
            return false;
        }
        if self.line.has_x_slope() && self.line.points()[0].y == other.points()[0].y {
            let min = self.line.points().iter().map(|p| p.x).min().unwrap();
            let max = self.line.points().iter().map(|p| p.x).max().unwrap();
            let other_min = other.points().iter().map(|p| p.x).min().unwrap();
            let other_max = other.points().iter().map(|p| p.x).max().unwrap();

            let is_min_outside = min > other_min || max < other_min;
            let is_max_outside = max < other_max;

            is_min_outside || is_max_outside
        } else if self.line.has_y_slope() && self.line.points()[0].x == other.points()[0].x {
            let min = self.line.points().iter().map(|p| p.y).min().unwrap();
            let max = self.line.points().iter().map(|p| p.y).max().unwrap();
            let other_min = other.points().iter().map(|p| p.y).min().unwrap();
            let other_max = other.points().iter().map(|p| p.y).max().unwrap();

            let is_min_outside = min > other_min || max < other_min;
            let is_max_outside = max < other_max;

            is_min_outside || is_max_outside
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::grid::Grid2D;

    use super::*;

    #[test]
    fn test() {
        let rect = generate_rect(IPoint::new(1, 1), IPoint::new(4, 4)).unwrap();

        let lines = [Line::new(IPoint::new(2, 2), IPoint::new(0, 2))];

        let mut grid = Grid2D::new(6, 6, '.');

        for line in &rect {
            line::draw(line, &mut grid, 'X');
        }

        line::draw(&lines[0], &mut grid, '#');

        println!("{}", grid);

        let intersected = rect
            .iter()
            .any(|line| lines.iter().any(|x| x.intersects(line)));
        assert!(intersected)
    }

    #[test]
    fn test3() {
        // 9,5 and 2,3
        let points = vec![
            IPoint::new(7, 1),
            IPoint::new(11, 1),
            IPoint::new(11, 7),
            IPoint::new(9, 7),
            IPoint::new(9, 5),
            IPoint::new(2, 5),
            IPoint::new(2, 3),
            IPoint::new(7, 3),
        ];

        let rect = generate_rect(IPoint::new(9, 5), IPoint::new(2, 3)).unwrap();

        let lines = generate_walls(&points);

        let mut grid = Grid2D::new(15, 10, '.');

        for (idx, wall) in lines.iter().enumerate() {
            line::draw(
                &wall.line,
                &mut grid,
                char::from_digit((idx + 1) as u32, 16).unwrap(),
            );
        }

        println!("{}", grid);

        let mut did_intersect = [false, false, false, false];

        for (idx, line) in rect.iter().enumerate() {
            for (line_idx, l) in lines.iter().enumerate() {
                if l.intersects(line) {
                    dbg!(l, line);
                    let c = char::from_digit(line_idx as u32, 16).unwrap();
                    println!("{c} intersects: {idx}")
                }
            }

            did_intersect[idx] = lines.iter().any(|x| x.intersects(line));
        }

        println!("{}", grid);

        assert!(!did_intersect[0]);
        assert!(!did_intersect[1]);
        assert!(!did_intersect[2]);
        assert!(!did_intersect[3]);
    }

    #[test]
    fn test4() {
        let points = vec![
            IPoint::new(7, 1),
            IPoint::new(11, 1),
            IPoint::new(11, 7),
            IPoint::new(9, 7),
            IPoint::new(9, 5),
            IPoint::new(2, 5),
            IPoint::new(2, 3),
            IPoint::new(7, 3),
        ];

        let walls = generate_walls(&points);

        // let mut grid = Grid2D::new(15, 10, '.');

        // for (idx, line) in lines.iter().enumerate() {
        //     line::draw(
        //         line,
        //         &mut grid,
        //         char::from_digit((idx + 1) as u32, 16).unwrap(),
        //     );
        // }

        let line = Line::new(IPoint::new(2, 1), IPoint::new(11, 1));

        assert!(!walls[0].intersects(&line));
        assert!(!walls[1].intersects(&line));
        assert!(walls[7].intersects(&line))
    }

    #[test]
    fn test5() {
        let wall = Wall::new(Line::new(IPoint::new(1, 1), IPoint::new(10, 1)));

        let middle_line = Line::new(IPoint::new(5, 5), IPoint::new(5, 0));

        assert!(wall.intersects(&middle_line));

        let outside_left_edge_line = Line::new(IPoint::new(0, 5), IPoint::new(0, 0));
        let outside_right_edge_line = Line::new(IPoint::new(11, 5), IPoint::new(11, 0));

        assert!(!wall.intersects(&outside_left_edge_line));
        assert!(!wall.intersects(&outside_right_edge_line));

        let left_edge_line = Line::new(IPoint::new(1, 5), IPoint::new(1, 0));
        let right_edge_line = Line::new(IPoint::new(10, 5), IPoint::new(10, 0));

        assert!(wall.intersects(&left_edge_line));
        assert!(wall.intersects(&right_edge_line));
    }
}
