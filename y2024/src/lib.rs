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

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
