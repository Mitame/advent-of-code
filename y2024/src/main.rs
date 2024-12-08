use std::{io::stdin, path::PathBuf, process::exit};

use advent_of_code::Aoc;
use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    day: usize,
    part: Option<usize>,
    input: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    for solution in inventory::iter::<Aoc> {
        if solution.day == args.day && args.part.map(|part| part == solution.part).unwrap_or(true) {
            return (solution.solver)(&mut stdin());
        }
    }

    eprintln!("Could not find solution for d{} p{:?}", args.day, args.part);
    exit(1)
}
