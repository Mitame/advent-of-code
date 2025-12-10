use std::io::{BufRead, BufReader, Read};

use crate::Aoc;

#[derive(Debug)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<u32>>,
    joltage_requirement: Vec<u32>,
}

fn parse_line(line: &str) -> Machine {
    let part_count = line.chars().filter(|c| c == &' ').count() + 1;
    let mut parts = line.split(" ");

    let lights: Vec<bool> = parts
        .next()
        .unwrap()
        .trim_matches(&['[', ']'])
        .chars()
        .map(|c| match c {
            '#' => true,
            '.' => false,
            _ => panic!("not a light: {}", c),
        })
        .collect();

    let buttons: Vec<Vec<u32>> = parts
        .clone()
        .take(part_count - 2)
        .map(|button| {
            button
                .trim_matches(&['(', ')'])
                .split(",")
                .map(|v| v.parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    let joltage_requirement: Vec<u32> = parts
        .last()
        .unwrap()
        .trim_matches(&['{', '}'])
        .split(",")
        .map(|v| v.parse::<u32>().unwrap())
        .collect();

    Machine {
        lights,
        buttons,
        joltage_requirement,
    }
}

fn parse(buf: &mut dyn Read) -> Vec<Machine> {
    let buf_reader = BufReader::new(buf);
    buf_reader
        .lines()
        .map(|line| parse_line(&line.unwrap()))
        .collect()
}

fn lights_to_mask(lights: &[bool]) -> u32 {
    lights.iter().enumerate().fold(0, |acc, (i, value)| {
        acc ^ value.then_some(1 << i).unwrap_or(0)
    })
}

fn button_to_mask(button: &[u32]) -> u32 {
    button
        .iter()
        .fold(0, |acc, light_index| acc ^ (1 << light_index))
}

fn binary_selector(i: u32, count: usize) -> impl Iterator<Item = usize> {
    (0..count).filter_map(move |b| (i & (1 << b) != 0).then_some(b as usize))
}

fn binary_select<'a, T>(i: u32, source: &'a [T]) -> Vec<&'a T> {
    binary_selector(i, source.len())
        .map(|j| &source[j])
        .collect()
}

fn button_combinations<'a>(buttons: &'a [Vec<u32>]) -> impl Iterator<Item = Vec<&'a Vec<u32>>> {
    (0..((2u32).pow(buttons.len() as u32))).map(|i| binary_select(i, buttons))
}

fn find_least_buttons_for_machine(machine: &Machine) -> usize {
    let target_mask = lights_to_mask(&machine.lights);
    button_combinations(&machine.buttons)
        .filter_map(|button_combination| {
            let button_count = button_combination.len();
            let result = button_combination
                .iter()
                .map(|button| button_to_mask(&button))
                .fold(0, |acc, mask| acc ^ mask);

            (result == target_mask).then_some(button_count)
        })
        .min()
        .unwrap()
}

fn part1(buf: &mut dyn Read) {
    let machines = parse(buf);
    let result: usize = machines
        .iter()
        .map(|machine| find_least_buttons_for_machine(machine))
        .sum();
    println!("Part 1: {}", result);
}

fn part2(buf: &mut dyn Read) {}

inventory::submit!(Aoc::new(2025, 10, part1, part2,));
