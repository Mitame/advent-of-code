use crate::Aoc;
use std::io::{BufRead, BufReader, Read};

use grid::{Grid, Location};

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

fn check_down(grid: &Grid<char>, search_text: &str, start: &Location) -> bool {
    let mut locations = vec![start.clone()];
    for _ in 1..search_text.len() {
        locations.push(locations.last().unwrap().down())
    }

    let target_str: Option<String> = locations
        .iter()
        .map(|location| grid.get(location))
        .collect();
    match target_str {
        None => false,
        Some(target) => target == search_text,
    }
}

fn check_right(grid: &Grid<char>, search_text: &str, start: &Location) -> bool {
    let mut locations = vec![start.clone()];
    for _ in 1..search_text.len() {
        let new_location = locations.last().unwrap().right();
        locations.push(new_location);
    }

    let target_str: Option<String> = locations
        .iter()
        .map(|location| grid.get(location))
        .collect();
    match target_str {
        None => false,
        Some(target) => target == search_text,
    }
}

fn check_diagonal_left(grid: &Grid<char>, search_text: &str, start: &Location) -> bool {
    let mut locations = vec![start.clone()];
    for _ in 1..search_text.len() {
        let new_location = locations.last().unwrap().down().left();
        if let Some(new_location) = new_location {
            locations.push(new_location);
        } else {
            return false;
        }
    }

    let target_str: Option<String> = locations
        .iter()
        .map(|location| grid.get(location))
        .collect();
    match target_str {
        None => false,
        Some(target) => target == search_text,
    }
}

fn check_diagonal_right(grid: &Grid<char>, search_text: &str, start: &Location) -> bool {
    let mut locations = vec![start.clone()];
    for _ in 1..search_text.len() {
        let new_location = locations.last().unwrap().down().right();
        locations.push(new_location);
    }

    let target_str: Option<String> = locations
        .iter()
        .map(|location| grid.get(location))
        .collect();
    match target_str {
        None => false,
        Some(target) => target == search_text,
    }
}

fn check_diagonal_up_right(grid: &Grid<char>, search_text: &str, start: &Location) -> bool {
    let mut locations = vec![start.clone()];
    for _ in 1..search_text.len() {
        let new_location = locations.last().unwrap().right().up();
        if let Some(new_location) = new_location {
            locations.push(new_location);
        } else {
            return false;
        }
    }

    let target_str: Option<String> = locations
        .iter()
        .map(|location| grid.get(location))
        .collect();
    match target_str {
        None => false,
        Some(target) => target == search_text,
    }
}

fn check_diagonal_up_left(grid: &Grid<char>, search_text: &str, start: &Location) -> bool {
    let mut locations = vec![start.clone()];
    for _ in 1..search_text.len() {
        let new_location = locations
            .last()
            .unwrap()
            .left()
            .and_then(|location| location.up());
        if let Some(new_location) = new_location {
            locations.push(new_location);
        } else {
            return false;
        }
    }

    let target_str: Option<String> = locations
        .iter()
        .map(|location| grid.get(location))
        .collect();
    match target_str {
        None => false,
        Some(target) => target == search_text,
    }
}

fn part1(buf: &mut dyn Read) {
    let body = parse(buf);
    let mut hit_count = 0;
    for location in body.iter_locations() {
        let hits = [
            check_down(&body, "XMAS", &location),
            check_right(&body, "XMAS", &location),
            check_diagonal_left(&body, "XMAS", &location),
            check_diagonal_right(&body, "XMAS", &location),
            check_down(&body, "SAMX", &location),
            check_right(&body, "SAMX", &location),
            check_diagonal_left(&body, "SAMX", &location),
            check_diagonal_right(&body, "SAMX", &location),
        ];

        let hits = hits.into_iter().filter(|hit| *hit).count();
        hit_count += hits
    }

    println!("Part 1: {}", hit_count);
}

fn part2(buf: &mut dyn Read) {
    let body = parse(buf);
    let mut hit_count = 0;
    for location in body.iter_locations() {
        let hit = check_diagonal_up_left(&body, "AS", &location)
            && check_diagonal_up_right(&body, "AS", &location)
            && check_diagonal_left(&body, "AM", &location)
            && check_diagonal_right(&body, "AM", &location)
            || check_diagonal_up_left(&body, "AM", &location)
                && check_diagonal_up_right(&body, "AS", &location)
                && check_diagonal_left(&body, "AM", &location)
                && check_diagonal_right(&body, "AS", &location)
            || check_diagonal_up_left(&body, "AM", &location)
                && check_diagonal_up_right(&body, "AM", &location)
                && check_diagonal_left(&body, "AS", &location)
                && check_diagonal_right(&body, "AS", &location)
            || check_diagonal_up_left(&body, "AS", &location)
                && check_diagonal_up_right(&body, "AM", &location)
                && check_diagonal_left(&body, "AS", &location)
                && check_diagonal_right(&body, "AM", &location);

        if hit {
            hit_count += 1;
        }
    }

    println!("Part 2: {}", hit_count);
}
inventory::submit!(Aoc::new(
    2024,
    4,
    part1,
    part2,
    include_bytes!("./inputs/day04")
));

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_check_right() {
        let grid = Grid::new("AAAAXMASAAAA".chars(), 4);
        assert!(check_right(&grid, "XMAS", &Location { x: 0, y: 1 }))
    }

    #[test]
    fn test_check_down() {
        let grid = Grid::new("AXAAMAAAAASA".chars(), 3);
        assert!(check_down(&grid, "XMAS", &Location { x: 1, y: 0 }))
    }

    #[test]
    fn test_check_diagonal_left() {
        let grid = Grid::new("ABCXDEMFGAHISJKL".chars(), 4);
        assert!(check_diagonal_left(&grid, "XMAS", &Location { x: 3, y: 0 }))
    }

    #[test]
    fn test_check_diagonal_right() {
        let grid = Grid::new("XBCDEMGHIJALMNOS".chars(), 4);
        assert!(check_diagonal_right(
            &grid,
            "XMAS",
            &Location { x: 0, y: 0 }
        ))
    }
}
