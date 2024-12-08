use std::{io::stdin, path::PathBuf, process::exit};

use advent_of_code::Aoc;
use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    year: usize,
    day: usize,
    part: usize,
    input: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    for solution in inventory::iter::<Aoc> {
        if solution.year == args.year && solution.day == args.day && args.part == solution.part {
            return (solution.solver)(&mut stdin());
        }
    }

    eprintln!("Could not find solution for d{} p{:?}", args.day, args.part);
    exit(1)
}
