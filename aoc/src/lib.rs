// #![feature(iterator_try_collect)]
// #![feature(iter_intersperse)]
use std::io;

type Solver = fn(&mut dyn io::Read);

pub struct Aoc {
    pub year: usize,
    pub day: usize,
    pub part1: Solver,
    pub part2: Solver,
}

impl Aoc {
    const fn new(year: usize, day: usize, part1: Solver, part2: Solver) -> Aoc {
        Aoc {
            year,
            day,
            part1,
            part2,
        }
    }

    pub fn get_input(&self) -> Box<dyn io::Read> {
        let file =
            std::fs::File::open(format!("inputs/y{}/day{:02}", self.year, self.day)).unwrap();
        Box::new(file)
    }
}

inventory::collect!(Aoc);

pub mod y2023;
pub mod y2024;
pub mod y2025;
