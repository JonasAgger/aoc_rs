use anyhow::Result;
use std::time::Instant;

use crate::events;
use tracing::{debug, info};

pub fn bench_day(day: u8, year: u16, input: Vec<String>, part: Option<u8>) -> Result<()> {
    info!("Running: Day {} Year {}", day, year);
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

    let start = Instant::now();
    let part1 = aoc_day.run_part1(&input)?;
    info!("--Part1: Took {:?}", start.elapsed());

    debug!("--Part1: '{}'", part1);
    Ok(())
}

fn part2(day: u8, year: u16, input: &Vec<String>) -> Result<()> {
    let mut aoc_day = events::get_day(day, year)?;

    let start = Instant::now();
    let part2 = aoc_day.run_part2(&input)?;
    info!("--Part2: Took {:?}", start.elapsed());

    debug!("--Part2: '{}'", part2);
    Ok(())
}
