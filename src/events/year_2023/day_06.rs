use std::ops::Neg;

use anyhow::Result;

use crate::utils::*;

use super::super::AocDay;

pub struct Day {}

impl Day {
    pub fn new() -> Self {
        Self {}
    }

    pub fn solve_for_zeros(&self, time: f64, dist: f64) -> (usize, usize) {
        let d = (time.powi(2) - (4f64 * -1f64 * dist.neg())).sqrt();
        let p = (time.neg() + &d) / -2f64;
        let n = (time.neg() - &d) / -2f64;

        // as these are ties, we need to inc/dec them slightly. this only matters if they're pure ints
        // Apparently ceil and floor are not high res enough to react to a single epsilon..
        let min = p.min(n) + f64::EPSILON * 10.0;
        let max = p.max(n) - f64::EPSILON * 10.0;

        (min.ceil() as usize, max.floor() as usize)
    }
}

impl AocDay for Day {
    fn run_part1(&mut self, input: &[String]) -> Result<AoCResult> {
        let times: Vec<f64> = input[0][5..]
            .split_ascii_whitespace()
            .filter_map(|t| t.trim().parse().ok())
            .collect();
        let distances: Vec<f64> = input[1][9..]
            .split_ascii_whitespace()
            .filter_map(|t| t.trim().parse().ok())
            .collect();

        let accum = times
            .iter()
            .zip(distances.iter())
            .map(|(&time, &dist)| self.solve_for_zeros(time, dist))
            .map(|(min, max)| (max - min) + 1) // have to push to be inclusive here
            .product::<usize>();

        Ok(accum.into())
    }

    fn run_part2(&mut self, input: &[String]) -> Result<AoCResult> {
        let time: f64 = input[0][5..].trim().replace(' ', "").parse()?;
        let distance: f64 = input[1][9..].trim().replace(' ', "").parse()?;

        let (min, max) = self.solve_for_zeros(time, distance);
        let possibilities = (max - min) + 1; // have to push to be inclusive here

        Ok(possibilities.into())
    }
}
