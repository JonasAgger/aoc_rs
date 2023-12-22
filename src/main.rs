#![feature(slice_group_by)]
#![feature(let_chains)]
#![feature(iter_map_windows)]
#![allow(dead_code)]

mod commands;
mod day_generator;
pub mod events;
pub mod utils;

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use tracing::{debug, Level};
use tracing_subscriber::FmtSubscriber;

use crate::{commands::InputFetcher, day_generator::DayGenerator};

const DAYS_FOLDER: &str = "./src/events";
const INPUT_FOLDER: &str = "./inputs";

#[derive(Parser, Debug)]
struct AoCOptions {
    /// Command
    #[arg(value_enum)]
    command: AoCCommands,

    /// Day to take action on
    day: Option<u8>,

    /// Day to take action on
    #[arg(short, long)]
    year: Option<u16>,

    /// Verbose Logging
    #[arg(short, long)]
    test: bool,

    /// Verbose Logging
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=2))]
    part: Option<u8>,

    /// Run with a specific input. Formatted day_<input>.txt
    #[arg(short, long)]
    input: Option<String>,

    /// Verbose Logging
    #[arg(short, long)]
    verbose: bool,

    /// Very Verbose Logging
    #[arg(long = "trace")]
    trace: bool,
}

#[derive(Subcommand, Debug, Clone, ValueEnum)]
enum AoCCommands {
    Run,
    Bench,
    Create,
    BenchAll,
}

fn main() -> Result<()> {
    let cli = AoCOptions::parse();

    let subscriber = FmtSubscriber::builder()
        .with_max_level(match (&cli.verbose, &cli.trace) {
            (_, true) => Level::TRACE,
            (true, false) => Level::DEBUG,
            (false, false) => Level::INFO,
        })
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let day_generator = DayGenerator::new(String::from(DAYS_FOLDER));

    let now = time::OffsetDateTime::now_utc();

    // Replace year and date

    let mut day = cli.day.unwrap_or_default();
    let mut year = cli.year.unwrap_or_default();

    if cli.year.is_none() {
        year = now.year() as u16;
    }

    if cli.day.is_none() {
        day = day_generator.get_current_day(year)?
    }

    debug!("{:?}", cli);

    match cli.command {
        AoCCommands::Run => {
            let mut input_fetcher = InputFetcher::new(INPUT_FOLDER);
            let input = input_fetcher.fetch(day, year, cli.test, &cli.input);
            commands::run::run_day(day, year, input, cli.part)
        }
        AoCCommands::Bench => {
            let mut input_fetcher = InputFetcher::new(INPUT_FOLDER);
            let input = input_fetcher.fetch(day, year, cli.test, &cli.input);
            commands::bench::bench_day(day, year, input, cli.part)
        }
        AoCCommands::BenchAll => {
            for day in 1..=day {
                let mut input_fetcher = InputFetcher::new(INPUT_FOLDER);
                let input = input_fetcher.fetch(day, year, cli.test, &cli.input);
                commands::bench::bench_day(day, year, input, cli.part)?;
            }
            Ok(())
        }
        AoCCommands::Create => day_generator.generate_day(
            match cli.day {
                Some(day) => day,
                None => day + 1,
            },
            year,
        ),
    }
}
