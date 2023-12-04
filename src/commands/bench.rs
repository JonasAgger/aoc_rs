use std::time::Instant;
use anyhow::Result;

use tracing::{info, debug};
use crate::events;



pub fn bench_day(day: u8, year: u16, input: Vec<String>) -> Result<()> {
    info!("Running: Day {} Year {}", day, year);

    let mut aoc_day = events::get_day(day, year)?;    

    let start = Instant::now();
    let part1 = aoc_day.run_part1(&input)?;
    info!("--Part1: Took {:?}", start.elapsed());
    
    debug!("--Part1: '{}'", part1);
    let mut aoc_day = events::get_day(day, year)?;    

    let start = Instant::now();
    let part2 = aoc_day.run_part2(&input)?;
    info!("--Part2: Took {:?}", start.elapsed());

    debug!("--Part2: '{}'", part2);

    Ok(())
}