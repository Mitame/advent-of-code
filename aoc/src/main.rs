use std::{path::PathBuf, process::exit, time::Instant};

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
            let part1_start = Instant::now();
            (solution.part1)(&mut input);
            let part1_time = part1_start.elapsed();
            eprintln!("(solved in {:?})", part1_time);
            
            let mut input = solution.input;
            let part2_start = Instant::now();
            (solution.part2)(&mut input);
            let part2_time = part2_start.elapsed();

            eprintln!("(solved in {:?})", part2_time);
            return;
        }
    }

    eprintln!("Could not find solution for y{} d{}", args.year, args.day);
    exit(1)
}
