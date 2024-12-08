use std::{path::PathBuf, process::exit};

use advent_of_code::Aoc;
use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    year: usize,
    day: usize,
    input: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    for solution in inventory::iter::<Aoc> {
        if solution.year == args.year && solution.day == args.day {
            let mut input = solution.input;
            (solution.part1)(&mut input);
            let mut input = solution.input;
            (solution.part2)(&mut input);
            return;
        }
    }

    eprintln!("Could not find solution for y{} d{}", args.year, args.day);
    exit(1)
}
