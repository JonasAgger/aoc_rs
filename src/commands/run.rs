use anyhow::Result;
use tracing::info;

use crate::events;

pub fn run_day(day: u8, year: u16, input: Vec<String>) -> Result<()> {
    let mut aoc_day = events::get_day(day, year)?;    

    let part1 = aoc_day.run_part1(&input)?;
    info!("Part1: {}", part1);
    
    let mut aoc_day = events::get_day(day, year)?;    

    let part2 = aoc_day.run_part2(&input)?;
    info!("Part2: {}", part2);

    Ok(())
}