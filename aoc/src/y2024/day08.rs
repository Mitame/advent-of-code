use grid::{Grid, Location, Offset};
use std::{
    collections::{HashMap, HashSet},
    io::{BufRead, BufReader, Read},
};

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

fn find_antennae_locations(grid: &Grid<char>) -> HashMap<char, HashSet<Location>> {
    let mut map: HashMap<char, HashSet<Location>> = HashMap::new();
    for location in grid.iter_locations() {
        let Some(content) = grid.get(&location) else {
            continue;
        };

        if content.is_alphanumeric() {
            map.entry(*content).or_default().insert(location);
        }
    }

    map
}

fn find_antinodes(a: &Location, b: &Location) -> Vec<Location> {
    let mut antinodes = Vec::new();

    let offset = b - a;

    if a.x as isize - offset.x >= 0 && a.y as isize - offset.y >= 0 {
        antinodes.push(a - &offset);
    }
    if b.x as isize + offset.x >= 0 && b.y as isize + offset.y >= 0 {
        antinodes.push(b + &offset);
    }

    antinodes
}

fn gcd(mut a: isize, mut b: isize) -> isize {
    let mut t;
    while b != 0 {
        t = b;
        b = a % b;
        a = t;
    }
    a
}

fn find_lax_antinodes(a: &Location, b: &Location) -> Vec<Location> {
    let mut antinodes = Vec::new();

    let offset = b - a;
    let divisor = gcd(offset.x, offset.y);
    let offset = Offset {
        x: offset.x / divisor,
        y: offset.y / divisor,
    };
    for i in 0..40 {
        let offset = &offset * i;
        if b.x as isize + offset.x >= 0 && b.y as isize + offset.y >= 0 {
            antinodes.push(b + &offset);
        }
        if a.x as isize - offset.x >= 0 && a.y as isize - offset.y >= 0 {
            antinodes.push(a - &offset);
        }
    }

    antinodes
}

fn find_pairs(locations: impl Iterator<Item = Location>) -> Vec<(Location, Location)> {
    let locations: Vec<Location> = locations.collect();
    let mut pairs = Vec::new();
    for (i, a) in locations.iter().enumerate() {
        for b in &locations[(i + 1)..] {
            pairs.push((a.clone(), b.clone()))
        }
    }

    pairs
}

fn part1(buf: &mut dyn Read) {
    let grid = parse(buf);
    let antennae_locations = find_antennae_locations(&grid);
    let antinode_locations: HashSet<Location> = antennae_locations
        .values()
        .flat_map(|locations| find_pairs(locations.iter().cloned()))
        .flat_map(|(a, b)| find_antinodes(&a, &b))
        .filter(|location| grid.is_within_bounds(location))
        .collect();

    println!("Part 1: {}", antinode_locations.len());
}

fn part2(buf: &mut dyn Read) {
    let grid = parse(buf);
    let antennae_locations = find_antennae_locations(&grid);
    let antinode_locations: HashSet<Location> = antennae_locations
        .values()
        .flat_map(|locations| find_pairs(locations.iter().cloned()))
        .flat_map(|(a, b)| find_lax_antinodes(&a, &b))
        .filter(|location| grid.is_within_bounds(location))
        .collect();

    println!("Part 2: {}", antinode_locations.len());
}

inventory::submit!(Aoc::new(
    2024,
    8,
    part1,
    part2,
    include_bytes!("./inputs/day08")
));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_antinodes() {
        assert_eq!(
            find_antinodes(&Location { x: 0, y: 0 }, &Location { x: 3, y: 3 }),
            vec![Location { x: 6, y: 6 },],
        );
        assert_eq!(
            find_antinodes(&Location { x: 1, y: 1 }, &Location { x: 2, y: 2 }),
            vec![Location { x: 0, y: 0 }, Location { x: 3, y: 3 },],
        );
        assert_eq!(
            find_antinodes(&Location { x: 2, y: 2 }, &Location { x: 1, y: 1 }),
            vec![Location { x: 3, y: 3 }, Location { x: 0, y: 0 },],
        );
        assert_eq!(
            find_antinodes(&Location { x: 1, y: 1 }, &Location { x: 2, y: 1 }),
            vec![Location { x: 0, y: 1 }, Location { x: 3, y: 1 }],
        );
        assert_eq!(
            find_antinodes(&Location { x: 1, y: 2 }, &Location { x: 1, y: 1 }),
            vec![Location { x: 1, y: 3 }, Location { x: 1, y: 0 },],
        );
    }
}
