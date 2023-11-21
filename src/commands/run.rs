use anyhow::Result;

use crate::tasks;

pub fn run_day(day: u8, year: u16) -> Result<()> {
    let mut day = tasks::get_day(day, year)?;    

    let part1 = day.run_part1()?;
    println!("Part1: {}", part1);
    
    let part2 = day.run_part2()?;
    println!("Part2: {}", part2);

    Ok(())
}