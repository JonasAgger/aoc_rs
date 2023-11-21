use std::time::{Duration, Instant};
use anyhow::Result;

use crate::tasks;



pub fn bench_day(day: u8, year: u16) -> Result<()> {
    let mut day = tasks::get_day(day, year)?;    

    let start = Instant::now();
    let part1 = day.run_part1()?;
    println!("Part1: Took {:?}", start.elapsed());
    
    let start = Instant::now();
    let part2 = day.run_part2()?;
    println!("Part2: Took {:?}", start.elapsed());

    Ok(())
}