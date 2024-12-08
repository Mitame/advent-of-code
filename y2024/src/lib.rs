#![feature(iterator_try_collect)]
use std::io;

type Solver = fn(&mut dyn io::Read);

pub struct Aoc {
    pub day: usize,
    pub part: usize,
    pub solver: Solver,
}

impl Aoc {
    const fn new(day: usize, part: usize, solver: Solver) -> Aoc {
        Aoc { day, part, solver }
    }
}

inventory::collect!(Aoc);

include!(concat!(env!("OUT_DIR"), "/days.rs"));
