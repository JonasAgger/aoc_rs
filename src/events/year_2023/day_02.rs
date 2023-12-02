use anyhow::Result;

use crate::utils::AoCResult;

use super::super::AocDay;


pub struct Day {
    red_max: usize,
    green_max: usize,
    blue_max: usize
}

impl Day {
    pub fn new() -> Self {
        Self {
            red_max: 12,
            green_max: 13,
            blue_max: 14
        }
    }

    fn part1_is_color_ok(&self, color_counts: Vec<&str>) -> Result<bool> {
        for i in (0..color_counts.len()).step_by(2) {
            let count: usize = color_counts[i].parse()?;
            let color = color_counts[i+1].trim_end_matches(|c| c == ';' || c == ',');
            match color {
                "red" if self.red_max < count => return Ok(false),
                "blue" if self.blue_max < count => return Ok(false),
                "green" if self.green_max < count => return Ok(false),
                _ => continue
            }
        }
        Ok(true)
    }

    fn part2_power_of_cubes(&self, color_counts: Vec<&str>) -> Result<usize> {
        let mut max_red = 0;
        let mut max_blue = 0;
        let mut max_green = 0;
        for i in (0..color_counts.len()).step_by(2) {
            let count: usize = color_counts[i].parse()?;
            let color = color_counts[i+1].trim_end_matches(|c| c == ';' || c == ',');
            match color {
                "red" if max_red < count => max_red = count,
                "blue" if max_blue < count => max_blue = count,
                "green" if max_green < count => max_green = count,
                _ => continue
            }
        }
        Ok(max_red * max_blue * max_green)
    }
}

impl AocDay for Day {
    fn run_part1(&mut self, input: &Vec<String>) -> Result<AoCResult> {
        let mut valid_games = vec![];

        for game in input {
            let mut parts = game.split(':');
            let id: i64 = parts.next().unwrap()[5..].parse()?; // Skip 'Game ' 

            let color_counts: Vec<_> = parts.last().unwrap()
                .split(' ')
                .filter(|s| s.len() > 0)
                .collect();
            
            if self.part1_is_color_ok(color_counts)? {
                valid_games.push(id);
            }
        }

        Ok(valid_games.into_iter().sum::<i64>().into())
    }

    fn run_part2(&mut self, input: &Vec<String>) -> Result<AoCResult> {
        let mut cube_powers = vec![];
        for game in input {
            let parts = game.split(':');
            let color_counts: Vec<_> = parts.last().unwrap()
                .split(' ')
                .filter(|s| s.len() > 0)
                .collect();
            
            cube_powers.push(self.part2_power_of_cubes(color_counts)?);
        }

        Ok(cube_powers.into_iter().sum::<usize>().into())
    }
}