#![feature(iterator_try_collect)]
use std::io;

type Solver = fn(&mut dyn io::Read);

pub struct Aoc {
    pub year: usize,
    pub day: usize,
    pub part: usize,
    pub solver: Solver,
}

impl Aoc {
    const fn new(year: usize, day: usize, part: usize, solver: Solver) -> Aoc {
        Aoc {
            year,
            day,
            part,
            solver,
        }
    }
}

inventory::collect!(Aoc);

pub mod y2024;
