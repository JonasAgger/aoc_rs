mod day_generator;
pub mod events;
mod commands;
pub mod utils;

use clap::{Parser, Subcommand, ValueEnum};
use anyhow::Result;
use tracing::{debug, Level};
use tracing_subscriber::FmtSubscriber;

use crate::{day_generator::DayGenerator, commands::InputFetcher};

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
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Subcommand, Debug, Clone, ValueEnum)]
enum AoCCommands {
    Run,
    Bench,
    Create,
    BenchAll
}

fn main() -> Result<()> {
    let mut cli = AoCOptions::parse();
    
    let subscriber = FmtSubscriber::builder()
        .with_max_level(match &cli.verbose {
            true => Level::DEBUG,
            false => Level::INFO,
        })
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let day_generator = DayGenerator::new(String::from(DAYS_FOLDER));

    let now = time::OffsetDateTime::now_utc();

    // Replace year and date
    if cli.year == None {
        cli.year = Some(now.year() as u16);
    }

    if cli.day == None {
        cli.day = Some(day_generator.get_current_day(cli.year.unwrap())?)
    }

    debug!("{:?}", cli);

    let day = cli.day.unwrap();
    let year = cli.year.unwrap();

    match cli.command {
        AoCCommands::Run => {
            let mut input_fetcher = InputFetcher::new(INPUT_FOLDER);
            let input = input_fetcher.fetch(day, year, cli.test);
            commands::run::run_day(day, year, input)
        },
        AoCCommands::Bench => {
            let mut input_fetcher = InputFetcher::new(INPUT_FOLDER);
            let input = input_fetcher.fetch(day, year, cli.test);
            commands::bench::bench_day(day, year, input)
        },
        AoCCommands::BenchAll => {
            for day in 1..=day {
                let mut input_fetcher = InputFetcher::new(INPUT_FOLDER);
                let input = input_fetcher.fetch(day, year, cli.test);
                commands::bench::bench_day(day, year, input)?;
            }
            Ok(())
        },
        AoCCommands::Create => day_generator.generate_day(day + 1, year),
    }
}
