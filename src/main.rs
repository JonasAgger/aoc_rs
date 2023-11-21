mod day_generator;
pub mod tasks;
mod commands;

use std::sync::atomic::AtomicBool;

use clap::{Parser, Subcommand, ValueEnum};
use anyhow::Result;

use crate::day_generator::DayGenerator;

const days_folder: &str = "./src/tasks";
static VERBOSE: AtomicBool = AtomicBool::new(false);

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
    verbose: bool,
}

#[derive(Subcommand, Debug, Clone, ValueEnum)]
enum AoCCommands {
    Run,
    Bench,
    Create
}

fn main() -> Result<()> {
    let mut cli = AoCOptions::parse();

    let day_generator = DayGenerator::new(String::from(days_folder));

    let now = time::OffsetDateTime::now_utc();

    // Replace year and date
    if cli.year == None {
        cli.year = Some(now.year() as u16);
    }

    if cli.day == None {
        cli.day = Some(day_generator.get_next_day(cli.year.unwrap())?)
    }

    println!("{:?}", cli);



    match cli.command {
        AoCCommands::Run => commands::run::run_day(cli.day.unwrap(), cli.year.unwrap()),
        AoCCommands::Bench => commands::bench::bench_day(cli.day.unwrap(), cli.year.unwrap()),
        AoCCommands::Create => day_generator.generate_day(cli.day.unwrap(), cli.year.unwrap()),
    }
    
}
