use anyhow::Result;

use crate::utils::{*, grid::Grid2D};

use super::super::AocDay;


pub struct Day {

}

impl Day {
    pub fn new() -> Self {
        Self {}
    }
}

impl AocDay for Day {
    fn run_part1(&mut self, input: &[String]) -> Result<AoCResult> {
        let galaxy = Grid2D::parse(input, |str| str.chars().map(|c| c == '#').collect());

        let x_offsets:Vec<_> = (0..galaxy.width()).filter(|&x| galaxy.get_col(x).into_iter().all(|&g| !g)).collect();
        let y_offsets:Vec<_> = (0..galaxy.height()).filter(|&y| galaxy.get_row(y).into_iter().all(|&g| !g)).collect();

        let mut galaxies = galaxy.find_all(|x| *x);

        for galaxy in galaxies.iter_mut() {

            let x_offset = x_offsets.iter().filter(|&&x| galaxy.x() > x).count();
            let y_offset = y_offsets.iter().filter(|&&y| galaxy.y() > y).count();

            *galaxy = Point::new(galaxy.x() + x_offset, galaxy.y() + y_offset);
        }

        let mut total_dist = 0;

        for i in 0..galaxies.len() {
            for j in i+1..galaxies.len() {
                total_dist += galaxies[i].manhattan_distance(&galaxies[j])
            }
        }

        Ok(total_dist.into())
    }

    fn run_part2(&mut self, input: &[String]) -> Result<AoCResult> {
        let galaxy = Grid2D::parse(input, |str| str.chars().map(|c| c == '#').collect());

        let x_offsets:Vec<_> = (0..galaxy.width()).filter(|&x| galaxy.get_col(x).into_iter().all(|&g| !g)).collect();
        let y_offsets:Vec<_> = (0..galaxy.height()).filter(|&y| galaxy.get_row(y).into_iter().all(|&g| !g)).collect();

        let mut galaxies = galaxy.find_all(|x| *x);

        for galaxy in galaxies.iter_mut() {

            let x_offset = x_offsets.iter().filter(|&&x| galaxy.x() > x).count() * 999_999;//1_000_000;
            let y_offset = y_offsets.iter().filter(|&&y| galaxy.y() > y).count() * 999_999;//1_000_000;

            *galaxy = Point::new(galaxy.x() + x_offset, galaxy.y() + y_offset);
        }

        let mut total_dist = 0;

        for i in 0..galaxies.len() {
            for j in i+1..galaxies.len() {
                total_dist += galaxies[i].manhattan_distance(&galaxies[j])
            }
        }

        Ok(total_dist.into())    }
}