use std::{
    collections::{HashSet, VecDeque},
    io::{BufRead, BufReader, Read},
};

use grid::{Grid, Location};

use crate::Aoc;

fn parse(buf: &mut dyn Read) -> Grid<char> {
    let reader = BufReader::new(buf);
    let mut lines = Vec::new();
    let mut line_length = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        line_length = line.len();
        lines.push(line);
    }
    Grid::new(lines.join("").chars(), line_length)
}

fn adjacent_locations(location: Location) -> Vec<Location> {
    [
        location.up(),
        location.right().up(),
        Some(location.right()),
        Some(location.right().down()),
        Some(location.down()),
        location.down().left(),
        location.left(),
        location.up().and_then(|location| location.left()),
    ]
    .into_iter()
    .flatten()
    .collect()
}

fn find_symbols(grid: &Grid<char>) -> impl Iterator<Item = Location> + '_ {
    grid.iter_locations().filter(|location| {
        let char = grid.get(location).unwrap();
        !char.is_numeric() && *char != '.'
    })
}

fn find_adjacent_numbers(
    grid: &Grid<char>,
    location: Location,
) -> impl Iterator<Item = (usize, Location)> + '_ {
    let adjacent_number_locations = adjacent_locations(location)
        .into_iter()
        .filter(|location| grid.get(location).map(|v| v.is_numeric()).unwrap_or(false));
    let mut scanned_cells = HashSet::new();

    adjacent_number_locations.filter_map(move |number_location| {
        if scanned_cells.contains(&number_location) {
            return None;
        }
        scanned_cells.insert(number_location.clone());

        let mut number_chars = VecDeque::new();
        number_chars.push_front(grid.get(&number_location).unwrap());
        let mut number_start = number_location.clone();
        while number_start
            .left()
            .and_then(|location| grid.get(&location).map(|char| char.is_numeric()))
            .unwrap_or(false)
        {
            number_start = number_start.left().unwrap();
            scanned_cells.insert(number_start.clone());
            number_chars.push_front(grid.get(&number_start).unwrap());
        }

        let mut number_end = number_location.clone();
        while grid
            .get(&number_end.right())
            .map(|char| char.is_numeric())
            .unwrap_or(false)
        {
            number_end = number_end.right();
            scanned_cells.insert(number_end.clone());
            number_chars.push_back(grid.get(&number_end).unwrap());
        }

        let number_string: String = number_chars.into_iter().collect();

        Some((number_string.parse::<usize>().unwrap(), number_start))
    })
}

fn part1(buf: &mut dyn Read) {
    let grid = parse(buf);
    let symbol_locations = find_symbols(&grid);

    let numbers: HashSet<_> = symbol_locations
        .flat_map(|location| find_adjacent_numbers(&grid, location))
        .collect();
    let result: usize = numbers.into_iter().map(|(v, _)| v).sum();

    println!("Part 1: {}", result);
}

fn part2(buf: &mut dyn Read) {
    let grid = parse(buf);
    let symbol_locations = find_symbols(&grid);

    let numbers = symbol_locations.filter_map(|location| {
        let adjacent_numbers: Vec<_> = find_adjacent_numbers(&grid, location)
            .map(|(v, _)| v)
            .collect();
        if adjacent_numbers.len() == 2 {
            Some(adjacent_numbers.into_iter().product::<usize>())
        } else {
            None
        }
    });
    let result: usize = numbers.sum();

    println!("Part 2: {}", result);
}

inventory::submit!(Aoc::new(
    2023,
    3,
    part1,
    part2,
    include_bytes!("./inputs/day03")
));
