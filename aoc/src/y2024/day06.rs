use crate::Aoc;
use grid::{Grid, Location};
use std::{
    collections::HashSet,
    io::{BufRead, BufReader, Read},
};

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

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn part1(buf: &mut dyn Read) {
    let grid = &parse(buf);
    let path = find_path(grid).unwrap();
    let set: HashSet<Location> = HashSet::from_iter(path.into_iter());
    println!("Part 1: {}", set.len());
}
inventory::submit!(Aoc::new(2024, 6, 1, part1));

fn find_path(grid: &Grid<char>) -> Option<Vec<Location>> {
    let grid = grid.clone();
    let mut direction = Direction::Up;
    let mut position = grid
        .iter_locations()
        .find(|location| grid.get(location) == Some(&'^'))
        .unwrap();

    let mut path = vec![position.clone()];
    let mut visited_cells = HashSet::new();
    visited_cells.insert((position.clone(), direction.clone()));

    while grid.get(&position).is_some() {
        let new_position = match direction {
            Direction::Up => position.up(),
            Direction::Right => Some(position.right()),
            Direction::Down => Some(position.down()),
            Direction::Left => position.left(),
        };

        let Some(new_position) = new_position else {
            break;
        };

        let peek = grid.get(&new_position);

        if peek == Some(&'#') {
            direction = match direction {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            };
        } else {
            path.push(new_position.clone());
            let inserted = visited_cells.insert((new_position.clone(), direction.clone()));
            if !inserted {
                return None;
            }
            position = new_position;
        }
    }

    Some(path)
}

fn check_for_loop(grid: &Grid<char>) -> bool {
    find_path(grid).is_none()
}

fn part2(buf: &mut dyn Read) {
    let grid = &parse(buf);

    let mut valid_obstruction_locations = HashSet::new();
    let path: HashSet<Location> = HashSet::from_iter(find_path(grid).unwrap().into_iter());
    for obstruction_location in path {
        // eprintln!("Checking {:?}", obstruction_location);
        let mut grid = grid.clone();
        if grid.get(&obstruction_location) == Some(&'#') {
            // Space is already occupied by an obstruction
            continue;
        }

        if grid.get(&obstruction_location) == Some(&'^') {
            // Space is already occupied by the start position
            continue;
        }

        grid.set(&obstruction_location, '#');

        let has_loop = check_for_loop(&grid);
        if has_loop {
            valid_obstruction_locations.insert(obstruction_location);
        }
    }

    println!("Part 2: {}", valid_obstruction_locations.len())
}

inventory::submit!(Aoc::new(2024, 6, 2, part2));
