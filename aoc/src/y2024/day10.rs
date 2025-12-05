use std::{
    collections::HashSet,
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

fn find_trailheads(grid: &Grid<char>) -> impl Iterator<Item = Location> + '_ {
    grid.iter_locations()
        .filter(|location| grid.get(location).map(|v| *v == '0').unwrap_or(false))
}

fn next_value(value: char) -> char {
    match value {
        '0' => '1',
        '1' => '2',
        '2' => '3',
        '3' => '4',
        '4' => '5',
        '5' => '6',
        '6' => '7',
        '7' => '8',
        '8' => '9',
        _ => panic!(),
    }
}

fn find_reachable_peaks(grid: &Grid<char>, location: &Location) -> Vec<Location> {
    let Some(value) = grid.get(location) else {
        return [].into();
    };
    if *value == '9' {
        return [location.clone()].into();
    }

    let expected_next_value = next_value(*value);

    let next_locations = [
        location.up(),
        Some(location.right()),
        Some(location.down()),
        location.left(),
    ]
    .into_iter()
    .flatten()
    .filter(|location| {
        grid.get(location)
            .map(|v| *v == expected_next_value)
            .unwrap_or(false)
    });

    next_locations
        .flat_map(|location| find_reachable_peaks(grid, &location))
        .collect()
}

fn part1(buf: &mut dyn Read) {
    let grid = parse(buf);
    let trailheads = find_trailheads(&grid);
    let result: usize = trailheads
        .map(|head| {
            find_reachable_peaks(&grid, &head)
                .into_iter()
                .collect::<HashSet<_>>()
                .len()
        })
        .sum();

    println!("Part 1: {}", result);
}

fn part2(buf: &mut dyn Read) {
    let grid = parse(buf);
    let trailheads = find_trailheads(&grid);
    let result: usize = trailheads
        .map(|head| find_reachable_peaks(&grid, &head).len())
        .sum();

    println!("Part 1: {}", result);
}

inventory::submit!(Aoc::new(2024, 10, part1, part2,));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_trailheads() {
        let grid = Grid::new(
            "8901012378121874874309659654987445678903320190120132980110456732".chars(),
            8,
        );
        let trailheads: Vec<_> = find_trailheads(&grid).collect();

        assert_eq!(
            trailheads,
            vec![
                Location { x: 2, y: 0 },
                Location { x: 4, y: 0 },
                Location { x: 4, y: 2 },
                Location { x: 6, y: 4 },
                Location { x: 2, y: 5 },
                Location { x: 5, y: 5 },
                Location { x: 0, y: 6 },
                Location { x: 6, y: 6 },
                Location { x: 1, y: 7 },
            ]
        );
    }

    #[test]
    fn test_find_reachable_peaks() {
        let grid: Grid<char> = Grid::new(
            "8901012378121874874309659654987445678903320190120132980110456732".chars(),
            8,
        );
        let peak_paths = find_reachable_peaks(&grid, &Location { x: 2, y: 0 });
        assert_eq!(peak_paths.len(), 20);
        assert_eq!(
            peak_paths.into_iter().collect::<HashSet<_>>(),
            HashSet::from([
                Location { x: 1, y: 0 },
                Location { x: 0, y: 3 },
                Location { x: 4, y: 3 },
                Location { x: 5, y: 4 },
                Location { x: 4, y: 5 },
            ])
        )
    }
}
