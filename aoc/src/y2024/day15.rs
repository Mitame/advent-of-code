use std::{
    fmt::Display,
    io::{BufRead, BufReader, Read},
};

use grid::{Direction, Grid, Location};

use crate::Aoc;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Cell {
    Empty,
    Robot,
    Box,
    Wall,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let char = match self {
            Cell::Empty => " ",
            Cell::Robot => "@",
            Cell::Box => "O",
            Cell::Wall => "#",
        };
        write!(f, "{}", char)
    }
}

fn parse_map(reader: &mut dyn BufRead) -> Grid<Cell> {
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
            '@' => Cell::Robot,
            'O' => Cell::Box,
            '#' => Cell::Wall,
            _ => panic!("Invalid map cell: {}", c),
        }),
        line_length,
    )
}

fn parse_moves(reader: &mut dyn BufRead) -> Vec<Direction> {
    let moves_str: String = reader
        .lines()
        .map_while(|line| {
            let line = line.unwrap();
            if line.is_empty() {
                None
            } else {
                Some(line)
            }
        })
        .collect();

    moves_str
        .chars()
        .map(|c| match c {
            '^' => Direction::Up,
            '>' => Direction::Right,
            'v' => Direction::Down,
            '<' => Direction::Left,
            _ => panic!("Invalid direction char: {}", c),
        })
        .collect()
}

struct Data {
    map: Grid<Cell>,
    moves: Vec<Direction>,
}

fn parse(buf: &mut dyn Read) -> Data {
    let mut reader: BufReader<_> = BufReader::new(buf);

    let map = parse_map(&mut reader);
    let moves = parse_moves(&mut reader);

    Data { map, moves }
}

fn perform_move(map: &mut Grid<Cell>, cell_location: &Location, direction: Direction) -> bool {
    let Some(cell) = map.get(cell_location) else {
        return false;
    };

    match cell {
        Cell::Wall => false,
        Cell::Empty => true,
        Cell::Robot | Cell::Box => {
            let cell = *cell;
            let Some(push_cell) = cell_location.to(direction) else {
                return false;
            };
            if perform_move(map, &push_cell, direction) {
                map.set(&push_cell, cell);
                map.set(cell_location, Cell::Empty);
                true
            } else {
                false
            }
        }
    }
}

fn part1(buf: &mut dyn Read) {
    let Data { mut map, moves } = parse(buf);

    let mut robot_location = map
        .iter_locations()
        .find(|location| *map.get(location).unwrap() == Cell::Robot)
        .unwrap();
    for direction in moves {
        let did_move = perform_move(&mut map, &robot_location, direction);
        if did_move {
            robot_location = robot_location.to(direction).unwrap();
        }
    }

    let result: usize = map
        .iter_locations()
        .filter_map(|location| {
            if *map.get(&location).unwrap() == Cell::Box {
                Some(location.y * 100 + location.x)
            } else {
                None
            }
        })
        .sum();

    println!("Part 1: {}", result);
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum WideCell {
    Empty,
    Robot,
    BoxLeft,
    BoxRight,
    Wall,
}

impl Display for WideCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let char = match self {
            WideCell::Empty => " ",
            WideCell::Robot => "@",
            WideCell::BoxLeft => "[",
            WideCell::BoxRight => "]",
            WideCell::Wall => "#",
        };
        write!(f, "{}", char)
    }
}

fn stretch_map(map: Grid<Cell>) -> Grid<WideCell> {
    let row_length = map.width() * 2;
    let wide_cells: Vec<_> = map
        .into_inner()
        .into_iter()
        .flat_map(|cell| match cell {
            Cell::Empty => [WideCell::Empty, WideCell::Empty],
            Cell::Robot => [WideCell::Robot, WideCell::Empty],
            Cell::Box => [WideCell::BoxLeft, WideCell::BoxRight],
            Cell::Wall => [WideCell::Wall, WideCell::Wall],
        })
        .collect();
    Grid::new(wide_cells, row_length)
}

fn perform_wide_move(
    map: &mut Grid<WideCell>,
    cell_location: &Location,
    direction: Direction,
    box_check: bool,
    move_partner: bool,
) -> bool {
    let Some(cell) = map.get(cell_location) else {
        return false;
    };

    match (cell, direction) {
        (WideCell::Wall, _) => false,
        (WideCell::Empty, _) => true,
        (WideCell::Robot, _)
        | (WideCell::BoxLeft, Direction::Left)
        | (WideCell::BoxLeft, Direction::Right)
        | (WideCell::BoxRight, Direction::Left)
        | (WideCell::BoxRight, Direction::Right) => {
            let cell = *cell;
            let Some(push_cell) = cell_location.to(direction) else {
                return false;
            };
            if perform_wide_move(map, &push_cell, direction, box_check, true) {
                if !box_check {
                    map.set(&push_cell, cell);
                    map.set(cell_location, WideCell::Empty);
                }
                true
            } else {
                false
            }
        }
        _ => {
            let cell = *cell;
            let Some(push_cell) = cell_location.to(direction) else {
                return false;
            };
            let other_cell_location = match cell {
                WideCell::BoxLeft => cell_location.right(),
                WideCell::BoxRight => cell_location.left().unwrap(),
                _ => return false,
            };

            let can_move = perform_wide_move(map, &push_cell, direction, true, true)
                && (!move_partner
                    || perform_wide_move(map, &other_cell_location, direction, true, false));
            if can_move && !box_check {
                perform_wide_move(map, &push_cell, direction, false, true);
                if move_partner {
                    perform_wide_move(map, &other_cell_location, direction, false, false);
                }
                map.set(&push_cell, cell);
                map.set(cell_location, WideCell::Empty);
            }

            can_move
        }
    }
}

fn part2(buf: &mut dyn Read) {
    let Data { map, moves } = parse(buf);
    let mut map = stretch_map(map);

    let mut robot_location = map
        .iter_locations()
        .find(|location| *map.get(location).unwrap() == WideCell::Robot)
        .unwrap();
    for direction in moves {
        let did_move = perform_wide_move(&mut map, &robot_location, direction, false, true);
        if did_move {
            robot_location = robot_location.to(direction).unwrap();
        }
    }

    eprintln!("--- Final Map ---");
    eprintln!("{}", map);

    let result: usize = map
        .iter_locations()
        .filter_map(|location| {
            if *map.get(&location).unwrap() == WideCell::BoxLeft {
                Some(location.y * 100 + location.x)
            } else {
                None
            }
        })
        .sum();
    println!("Part 2: {}", result);
}

inventory::submit!(Aoc::new(2024, 15, part1, part2,));
