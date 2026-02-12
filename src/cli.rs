use clap::Parser;

use crate::puzzles::{Day, Puzzle, Year};

#[derive(Debug, Parser)]
#[command(about)]
struct Cli {
    #[arg(short, long)]
    year: Option<Year>,

    #[arg(short, long)]
    day: Option<Day>,
}

pub fn parse_args() -> Vec<Puzzle> {
    let Cli { year, day } = Cli::parse();
    let years = year.map(|y| vec![y]).unwrap_or(Year::all_years());
    years
        .into_iter()
        .flat_map(|year| {
            let days = day.clone().map(|d| vec![d]).unwrap_or(year.all_days());
            days.into_iter()
                .map(|day| Puzzle::new(year.clone(), day.clone()))
                .collect::<Vec<_>>()
        })
        .collect()
}
