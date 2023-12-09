use anyhow::Result;
use tracing::info;

use crate::events;

pub fn run_day(day: u8, year: u16, input: Vec<String>, part: Option<u8>) -> Result<()> {
    match part {
        Some(1) => part1(day, year, &input),
        Some(2) => part2(day, year, &input),
        _ => {
            part1(day, year, &input)?;
            part2(day, year, &input)
        }
    }
}

fn part1(day: u8, year: u16, input: &Vec<String>) -> Result<()> {
    let mut aoc_day = events::get_day(day, year)?;

    let part1 = aoc_day.run_part1(input)?;
    info!("Part1: {}", part1);
    Ok(())
}

fn part2(day: u8, year: u16, input: &Vec<String>) -> Result<()> {
    let mut aoc_day = events::get_day(day, year)?;

    let part2 = aoc_day.run_part2(input)?;
    info!("Part2: {}", part2);
    Ok(())
}
