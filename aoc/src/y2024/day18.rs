use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
};

use grid::{Direction, Grid, Location};
use priority_queue::PriorityQueue;

use crate::Aoc;

fn parse(buf: &mut dyn Read) -> Vec<Location> {
    let reader = BufReader::new(buf);
    reader
        .lines()
        .map_while(|line| {
            let line = line.ok()?;
            let (x, y) = line.split_once(',')?;
            Some(Location {
                x: x.parse().ok()?,
                y: y.parse().ok()?,
            })
        })
        .collect()
}

fn solve_maze<T>(
    grid: &Grid<T>,
    move_check: impl Fn(&Location, &Grid<T>) -> bool,
    start: &Location,
    end: &Location,
) -> Option<Vec<Location>> {
    let mut open_set: PriorityQueue<Location, usize> = [(start.clone(), 0)].into_iter().collect();
    let mut came_from: HashMap<Location, Location> = HashMap::new();
    let mut g_score: HashMap<Location, usize> = HashMap::new();
    g_score.insert(start.clone(), 0);

    let mut f_score: HashMap<Location, usize> = HashMap::new();
    f_score.insert(start.clone(), start.manhattan_distance(end));

    while let Some((current, _)) = open_set.pop() {
        if &current == end {
            let mut route = vec![];
            let mut current = Some(&current);
            while let Some(loc) = current {
                route.push(loc.clone());
                current = came_from.get(loc);
            }
            return Some(route);
        }

        let neighbours = [
            Direction::Up,
            Direction::Down,
            Direction::Right,
            Direction::Left,
        ]
        .into_iter()
        .filter_map(|d| current.to(d))
        .filter(|loc| move_check(loc, grid));
        for neighbour in neighbours {
            let new_g_score = g_score.get(&current).unwrap_or(&usize::MAX) + 1;
            if new_g_score < *g_score.get(&neighbour).unwrap_or(&usize::MAX) {
                came_from.insert(neighbour.clone(), current.clone());
                g_score.insert(neighbour.clone(), new_g_score);
                let new_f_score = new_g_score + neighbour.manhattan_distance(end);
                f_score.insert(neighbour.clone(), new_f_score);
                open_set.push(neighbour, new_f_score);
            }
        }
    }

    None
}

fn part1(buf: &mut dyn Read) {
    let corruption_positions = parse(buf);

    let mut grid = Grid::new([' '].into_iter().cycle().take(71 * 71), 71);
    for corruption_position in corruption_positions.into_iter().take(1024) {
        grid.set(&corruption_position, '#');
    }

    let route = solve_maze(
        &grid,
        |location, grid| grid.get(location).map(|cell| *cell != '#').unwrap_or(false),
        &Location { x: 0, y: 0 },
        &Location { x: 70, y: 70 },
    );

    for pos in route.as_ref().unwrap() {
        grid.set(pos, 'O');
    }
    let result = route.unwrap().len() - 1;
    println!("Part 1: {}", result);
}

fn part2(buf: &mut dyn Read) {
    let corruption_positions = parse(buf);

    let mut grid = Grid::new([' '].into_iter().cycle().take(71 * 71), 71);
    let mut route = solve_maze(
        &grid,
        |location, grid| grid.get(location).map(|cell| *cell != '#').unwrap_or(false),
        &Location { x: 0, y: 0 },
        &Location { x: 70, y: 70 },
    )
    .unwrap();
    for corruption_position in corruption_positions {
        grid.set(&corruption_position, '#');
        if route.contains(&corruption_position) {
            // Recalculate route
            if let Some(solution) = solve_maze(
                &grid,
                |location, grid| grid.get(location).map(|cell| *cell != '#').unwrap_or(false),
                &Location { x: 0, y: 0 },
                &Location { x: 70, y: 70 },
            ) {
                route = solution
            } else {
                println!(
                    "Part 2: {},{}",
                    corruption_position.x, corruption_position.y
                );
                return;
            }
        }
    }
}

inventory::submit!(Aoc::new(2024, 18, part1, part2,));
