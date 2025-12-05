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

fn find_plots(garden: &Grid<char>) -> Vec<Vec<Location>> {
    let mut checked_locations: HashSet<Location> = HashSet::new();
    let mut plots = Vec::new();
    for location in garden.iter_locations() {
        if checked_locations.contains(&location) {
            continue;
        }

        let plot_type = garden.get(&location).unwrap();

        let mut plot: Vec<Location> = Vec::new();
        let mut locations_to_check = vec![location];
        while let Some(location) = locations_to_check.pop() {
            if checked_locations.contains(&location) {
                continue;
            }
            let Some(cell_type) = garden.get(&location) else {
                continue;
            };
            if cell_type != plot_type {
                continue;
            }

            plot.push(location.clone());
            checked_locations.insert(location.clone());

            if let Some(location) = location.up() {
                locations_to_check.push(location)
            }
            locations_to_check.push(location.down());
            if let Some(location) = location.left() {
                locations_to_check.push(location)
            }
            locations_to_check.push(location.right());
        }

        plots.push(plot);
    }

    plots
}

fn find_perimeter(plot: Vec<Location>) -> usize {
    let locations: HashSet<_> = plot.iter().collect();

    let mut perimeter = locations.len() * 4;
    for location in locations.iter() {
        if let Some(neighbour) = location.up() {
            if locations.contains(&neighbour) {
                perimeter -= 1;
            }
        }
        if let Some(neighbour) = location.left() {
            if locations.contains(&neighbour) {
                perimeter -= 1;
            }
        }
        if locations.contains(&location.right()) {
            perimeter -= 1;
        }
        if locations.contains(&location.down()) {
            perimeter -= 1;
        }
    }

    perimeter
}

fn count_corners(cell: Location, grid: &Grid<char>) -> usize {
    // Check cardinal directions
    // (yes, this is truly awful)
    let centre = grid.get(&cell).unwrap();
    let up = cell
        .up()
        .and_then(|location| grid.get(&location))
        .unwrap_or(&' ');
    let right = grid.get(&cell.right()).unwrap_or(&' ');
    let down = grid.get(&cell.down()).unwrap_or(&' ');
    let left = cell
        .left()
        .and_then(|location| grid.get(&location))
        .unwrap_or(&' ');
    let up_left = cell
        .up()
        .and_then(|loc| loc.left())
        .and_then(|loc| grid.get(&loc))
        .unwrap_or(&' ');
    let up_right = cell
        .right()
        .up()
        .and_then(|loc| grid.get(&loc))
        .unwrap_or(&' ');
    let down_right = grid.get(&cell.right().down()).unwrap_or(&' ');
    let down_left = cell
        .down()
        .left()
        .and_then(|loc| grid.get(&loc))
        .unwrap_or(&' ');

    let mut corner_count = 0;
    let rotations = [
        (up == centre, up_right == centre, right == centre),
        (left == centre, up_left == centre, up == centre),
        (down == centre, down_left == centre, left == centre),
        (right == centre, down_right == centre, down == centre),
    ];
    for rot in rotations {
        match rot {
            (true, false, true) | (false, _, false) => corner_count += 1,
            _ => {}
        }
    }

    corner_count
}

fn find_corners(plot: Vec<Location>, grid: &Grid<char>) -> usize {
    plot.into_iter()
        .map(|location| count_corners(location, grid))
        .sum()
}

fn find_area(plot: Vec<Location>) -> usize {
    plot.len()
}

fn part1(buf: &mut dyn Read) {
    let garden = parse(buf);

    let plots = find_plots(&garden);
    let result: usize = plots
        .into_iter()
        .map(|plot| {
            let perimeter = find_perimeter(plot.clone());
            let area = find_area(plot);
            perimeter * area
        })
        .sum();

    println!("Part 1: {}", result);
}

fn part2(buf: &mut dyn Read) {
    let garden = parse(buf);

    let plots = find_plots(&garden);

    let result: usize = plots
        .into_iter()
        .map(|plot| {
            let area = find_area(plot.clone());
            let corners = find_corners(plot, &garden);
            area * corners
        })
        .sum();

    println!("Part 2: {}", result);
}

inventory::submit!(Aoc::new(2024, 12, part1, part2,));
