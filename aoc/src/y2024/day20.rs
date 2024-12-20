use std::{
    collections::{BTreeMap, HashMap, HashSet, VecDeque},
    fmt::Display,
    io::{BufRead, BufReader, Read},
};

use grid::{Direction, Grid, Location};

use crate::Aoc;
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Cell {
    Empty,
    Start,
    End,
    Wall,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let char = match self {
            Cell::Empty => " ",
            Cell::Start => "S",
            Cell::End => "E",
            Cell::Wall => "#",
        };
        write!(f, "{}", char)
    }
}

fn parse(buf: &mut dyn Read) -> Grid<Cell> {
    let reader = BufReader::new(buf);
    let mut line_length = 0;
    let lines = reader.lines().map_while(|line| {
        let line = line.unwrap();
        if line.is_empty() {
            None
        } else {
            line_length = line.len();
            Some(line)
        }
    });
    Grid::new(
        lines.collect::<String>().chars().map(|c| match c {
            '.' => Cell::Empty,
            'S' => Cell::Start,
            'E' => Cell::End,
            '#' => Cell::Wall,
            _ => panic!("Invalid map cell: {}", c),
        }),
        line_length,
    )
}

fn find_cell_distances(maze: &Grid<Cell>, from_location: Location) -> HashMap<Location, usize> {
    let mut queue = VecDeque::from([from_location.clone()]);
    let mut distance_map = HashMap::from([(from_location, 0)]);

    while let Some(location) = queue.pop_front() {
        let distance = *distance_map.get(&location).unwrap();
        let neighbours = [
            Direction::Up,
            Direction::Left,
            Direction::Down,
            Direction::Right,
        ]
        .into_iter()
        .filter_map(|direction| location.to(direction))
        .filter(|location| {
            maze.get(location)
                .map(|cell| *cell != Cell::Wall)
                .unwrap_or(false)
        });
        neighbours.for_each(|location| {
            if !distance_map.contains_key(&location) {
                distance_map.insert(location.clone(), distance + 1);
                queue.push_back(location);
            }
        });
    }

    distance_map
}

fn find_cheat_routes(
    start_location: &Location,
    cell_distances_to_end: &HashMap<Location, usize>,
    max_cheat_length: usize,
) -> HashMap<(Location, Location), usize> {
    let start_distance = *cell_distances_to_end.get(&start_location).unwrap();

    // Remove all cell distances that are futher away from the end than the start, as we don't want to cheat from those routes.
    let cell_distances_to_end: HashMap<_, _> = cell_distances_to_end
        .iter()
        .filter(|(location, distance)| *distance < &start_distance || location == &start_location)
        .map(|(k, v)| (k.clone(), *v))
        .collect();

    cell_distances_to_end
        // Get the product of the cell distances with itself, limited only to
        .iter()
        .map(|a| [a].into_iter().cycle().zip(cell_distances_to_end.iter()))
        .flatten()
        // Limit to items where the cheat is permitted
        .filter(|((a_location, a_distance), (b_location, b_distance))| {
            a_distance > b_distance && a_location.manhattan_distance(b_location) <= max_cheat_length
        })
        // Make the key into (from, to) and the value into (difference_in_distance)
        .map(|((a_location, a_distance), (b_location, b_distance))| {
            (
                (a_location.clone(), b_location.clone()),
                a_distance.abs_diff(*b_distance) - a_location.manhattan_distance(b_location),
            )
        })
        .collect()
}

fn part1(buf: &mut dyn Read) {
    let maze = parse(buf);

    let start_position = maze
        .iter_locations()
        .find(|v| maze.get(v) == Some(&Cell::Start))
        .unwrap();
    let end_position = maze
        .iter_locations()
        .find(|v| maze.get(v) == Some(&Cell::End))
        .unwrap();

    let cell_distances = find_cell_distances(&maze, end_position);
    let start_distance = *cell_distances.get(&start_position).unwrap();

    // Don't allow backtracking
    let cell_distances: HashMap<_, _> = cell_distances
        .into_iter()
        .filter(|(location, distance)| distance < &start_distance || location == &start_position)
        .collect();

    let cheat_locations = cell_distances
        .iter()
        .filter(|(_, distance)| *distance < &start_distance)
        .map(|(location, distance)| {
            [
                Direction::Up,
                Direction::Left,
                Direction::Down,
                Direction::Right,
            ]
            .into_iter()
            .filter_map(|direction| {
                location
                    .to(direction)
                    .and_then(|location| location.to(direction))
            })
            .filter(|location| {
                maze.get(location)
                    .map(|cell| *cell != Cell::Wall)
                    .unwrap_or(false)
            })
            .filter_map(|cheat_end_location| {
                cell_distances
                    .get(&cheat_end_location)
                    .map(|cheat_end_distance| {
                        (
                            (location.clone(), *distance),
                            (cheat_end_location, *cheat_end_distance),
                        )
                    })
            })
            .filter_map(|(a, b)| {
                let diff = a.1.abs_diff(b.1) - 2;
                if diff == 0 {
                    None
                } else {
                    // Find the distance closest to the start, which will be furthest from the end
                    if a.1 > b.1 {
                        Some((a.0, diff))
                    } else {
                        Some((b.0, diff))
                    }
                }
            })
        })
        .flatten()
        .fold(
            HashMap::<usize, HashSet<Location>>::new(),
            |mut acc, (location, distance)| {
                acc.entry(distance).or_default().insert(location);
                acc
            },
        );

    // let mut cheat_distances: Vec<_> = cheat_locations.keys().collect();
    // cheat_distances.sort();
    // for distance in cheat_distances {
    //     eprintln!("There are {} cheats that save {} picoseconds.", cheat_locations.get(distance).unwrap().len(), distance);
    //     dbg!(cheat_locations.get(distance).unwrap());
    // }

    let result: usize = cheat_locations
        .iter()
        .filter_map(|(k, v)| (*k >= 100).then_some(v.len()))
        .sum();

    println!("Part 1: {}", result);
}

fn part2(buf: &mut dyn Read) {
    let maze = parse(buf);

    let start_location = maze
        .iter_locations()
        .find(|v| maze.get(v) == Some(&Cell::Start))
        .unwrap();
    let end_location = maze
        .iter_locations()
        .find(|v| maze.get(v) == Some(&Cell::End))
        .unwrap();

    let cell_distances_to_end = find_cell_distances(&maze, end_location);

    let cheat_routes: HashMap<(Location, Location), usize> =
        find_cheat_routes(&start_location, &cell_distances_to_end, 20);

    let valid_cheat_routes: HashSet<(Location, Location)> = cheat_routes
        .into_iter()
        .filter_map(|(k, v)| (v >= 100).then_some(k))
        .collect();
    let result = valid_cheat_routes.len();
    println!("Part 2: {}", result);
}

inventory::submit!(Aoc::new(
    2024,
    20,
    part1,
    part2,
    include_bytes!("./inputs/day20")
));
