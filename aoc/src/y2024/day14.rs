use std::io::{BufRead, BufReader, Read};

use grid::{Grid, Location, Offset};

use crate::Aoc;

#[derive(Debug, PartialEq)]
struct Robot {
    position: Location,
    velocity: Offset,
}

fn parse(buf: &mut dyn Read) -> Vec<Robot> {
    let reader = BufReader::new(buf);
    reader
        .lines()
        .map_while(|line| {
            let Ok(line) = line else { return None };

            let (position, velocity) = line.split_once(' ').unwrap();
            let mut position = position
                .strip_prefix("p=")
                .unwrap()
                .split(",")
                .map(|v| v.parse::<usize>().unwrap());
            let position = Location {
                x: position.next().unwrap(),
                y: position.next().unwrap(),
            };

            let mut velocity = velocity
                .strip_prefix("v=")
                .unwrap()
                .split(",")
                .map(|v| v.parse::<isize>().unwrap());
            let velocity = Offset {
                x: velocity.next().unwrap(),
                y: velocity.next().unwrap(),
            };

            Some(Robot { position, velocity })
        })
        .collect()
}

fn simulate_movement(robot: &Robot, t: isize, width: usize, height: usize) -> Robot {
    let displacement = &robot.velocity * t;
    let position = &Offset {
        x: robot.position.x as isize,
        y: robot.position.y as isize,
    } + &displacement;
    let position = Location {
        x: (((position.x % width as isize) + width as isize) % width as isize) as usize,
        y: (((position.y % height as isize) + height as isize) % height as isize) as usize,
    };
    Robot {
        position,
        velocity: robot.velocity.clone(),
    }
}

fn part1(buf: &mut dyn Read) {
    let robots = parse(buf);

    let width = 101;
    let height = 103;

    let moved_robots = robots
        .iter()
        .map(|robot| simulate_movement(robot, 100, width, height));
    let locations: Vec<_> = moved_robots.map(|r| r.position).collect();
    let quads = [
        (0..width / 2, 0..height / 2),
        ((width / 2 + 1)..width, 0..height / 2),
        ((width / 2 + 1)..width, (height / 2 + 1)..height),
        (0..width / 2, (height / 2 + 1)..height),
    ];
    let counts: Vec<usize> = quads
        .into_iter()
        .map(|(x_range, y_range)| {
            locations
                .iter()
                .filter(|loc| x_range.contains(&loc.x) && y_range.contains(&loc.y))
                .count()
        })
        .collect();

    let result: usize = counts.into_iter().product();

    println!("Part 1: {}", result);
}

fn part2(buf: &mut dyn Read) {
    let robots = parse(buf);

    let width = 101;
    let height = 103;

    let base_grid = Grid::new([' '].iter().cycle().take(width * height), width);
    for i in 0..10000 {
        let mut grid = base_grid.clone();
        robots
            .iter()
            .map(|robot| simulate_movement(robot, i, width, height))
            .for_each(|robot| {
                grid.set(&robot.position, &'O');
            });
        let cells: String = grid.cells().iter().cloned().collect();
        // The image contains some lines. 10 'O's should be enough
        if cells.contains("OOOOOOOOO") {
            eprintln!("{}", grid);
            println!("Part 2: {}", i);
            return;
        }
    }
}

inventory::submit!(Aoc::new(
    2024,
    14,
    part1,
    part2,
    include_bytes!("./inputs/day14")
));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_robot() {
        let robot = Robot {
            position: Location { x: 2, y: 4 },
            velocity: Offset { x: 2, y: -3 },
        };
        assert_eq!(
            simulate_movement(&robot, 1, 11, 7),
            Robot {
                position: Location { x: 4, y: 1 },
                velocity: robot.velocity.clone()
            }
        );
        assert_eq!(
            simulate_movement(&robot, 2, 11, 7),
            Robot {
                position: Location { x: 6, y: 5 },
                velocity: robot.velocity.clone()
            }
        );
        assert_eq!(
            simulate_movement(&robot, 3, 11, 7),
            Robot {
                position: Location { x: 8, y: 2 },
                velocity: robot.velocity.clone()
            }
        );
        assert_eq!(
            simulate_movement(&robot, 4, 11, 7),
            Robot {
                position: Location { x: 10, y: 6 },
                velocity: robot.velocity.clone()
            }
        );
        assert_eq!(
            simulate_movement(&robot, 5, 11, 7),
            Robot {
                position: Location { x: 1, y: 3 },
                velocity: robot.velocity.clone()
            }
        );
    }
}
