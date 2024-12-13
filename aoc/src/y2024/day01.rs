use crate::Aoc;
use regex::Regex;
use std::io::BufRead;
use std::{collections::HashMap, io::BufReader};

struct Data {
    list_a: Vec<u32>,
    list_b: Vec<u32>,
}

fn parse(buf: &mut dyn std::io::Read) -> Data {
    let mut list_a = Vec::<u32>::new();
    let mut list_b = Vec::<u32>::new();

    let reader = BufReader::new(buf);
    let separator_re = Regex::new(" +").unwrap();
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        let [a, b] = separator_re
            .split(&line)
            .map(|v| v.parse::<u32>().unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        list_a.push(a);
        list_b.push(b);
    }

    Data { list_a, list_b }
}

fn part1(buf: &mut dyn std::io::Read) {
    let Data {
        mut list_a,
        mut list_b,
    } = parse(buf);
    list_a.sort();
    list_b.sort();

    let result: u32 = list_a
        .into_iter()
        .zip(list_b)
        .map(|(a, b)| a.abs_diff(b))
        .sum();
    println!("Part 1: {}", result);
}

fn count(list: Vec<u32>) -> HashMap<u32, u32> {
    let mut counter = HashMap::new();

    for item in list {
        *counter.entry(item).or_default() += 1;
    }

    counter
}

fn part2(buf: &mut dyn std::io::Read) {
    let Data { list_a, list_b } = parse(buf);

    let count_a = count(list_a);
    let count_b = count(list_b);

    let score: u32 = count_a
        .into_iter()
        .map(|(n, count)| n * count * count_b.get(&n).cloned().unwrap_or(0))
        .sum();
    println!("Part 2: {}", score);
}
inventory::submit!(Aoc::new(
    2024,
    1,
    part1,
    part2,
    include_bytes!("./inputs/day01")
));

// fn main() {
//     part1(list_a.clone(), list_b.clone());

//     part2(list_a, list_b);
// }
