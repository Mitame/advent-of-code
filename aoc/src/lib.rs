#![feature(iterator_try_collect)]
#![feature(iter_intersperse)]
use std::io;

type Solver = fn(&mut dyn io::Read);

pub struct Aoc {
    pub year: usize,
    pub day: usize,
    pub part1: Solver,
    pub part2: Solver,
    pub input: &'static [u8],
}

impl Aoc {
    const fn new(
        year: usize,
        day: usize,
        part1: Solver,
        part2: Solver,
        input: &'static [u8],
    ) -> Aoc {
        Aoc {
            year,
            day,
            part1,
            part2,
            input,
        }
    }
}

inventory::collect!(Aoc);

pub mod y2023;
pub mod y2024;
