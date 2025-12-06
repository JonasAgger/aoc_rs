use anyhow::Result;
use std::time::{Duration, Instant};

use crate::events;
use tracing::{debug, info};

const ITERATION_CUTOFF: u32 = 100;
const TIME_CUTOFF: Duration = Duration::from_secs(1);

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
    let mut iterations = 0;
    let part1 = loop {
        let part1 = std::hint::black_box(aoc_day.run_part1(input)?);
        iterations += 1;
        if start.elapsed() > TIME_CUTOFF || iterations > ITERATION_CUTOFF {
            break part1;
        }
    };

    info!("--Part1: Took {:?}", start.elapsed() / iterations);

    debug!("--Part1: '{}'", part1);
    Ok(())
}

fn part2(day: u8, year: u16, input: &Vec<String>) -> Result<()> {
    let mut aoc_day = events::get_day(day, year)?;

    let start = Instant::now();
    let mut iterations = 0;
    let part2 = loop {
        let part2 = std::hint::black_box(aoc_day.run_part2(input)?);
        iterations += 1;
        if start.elapsed() > TIME_CUTOFF || iterations > ITERATION_CUTOFF {
            break part2;
        }
    };
    info!("--Part2: Took {:?}", start.elapsed() / iterations);

    debug!("--Part2: '{}'", part2);
    Ok(())
}
