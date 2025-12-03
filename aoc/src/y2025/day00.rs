use std::io::Read;

use crate::Aoc;

fn part1(buf: &mut dyn Read) {}

fn part2(buf: &mut dyn Read) {}

inventory::submit!(Aoc::new(
    2025,
    0,
    part1,
    part2,
    include_bytes!("./inputs/day00")
));
