mod year_2023;


use anyhow::Result;

pub trait AocDay {
    fn run_part1(&mut self) -> Result<String>; 
    fn run_part2(&mut self) -> Result<String>; 
}

pub fn get_day(day: u8, year: u16) -> Result<Box<dyn AocDay>> {
    match (day, year) {
        (1, 2023) => Ok(Box::new(year_2023::day_01::Day::new())),

        _ => anyhow::bail!("Received invalid day: {}", day) 
    }
} 