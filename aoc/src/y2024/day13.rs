use std::io::{BufRead, BufReader, Read};

use grid::{Location, Offset};

use crate::Aoc;

#[derive(Debug, Clone)]
struct Machine {
    button_a: Offset,
    button_b: Offset,
    prize_position: Location,
}

fn parse(buf: &mut dyn Read) -> Vec<Machine> {
    let reader = BufReader::new(buf);

    let mut machines = Vec::new();
    let mut machine = Machine {
        button_a: Offset { x: 0, y: 0 },
        button_b: Offset { x: 0, y: 0 },
        prize_position: Location { x: 0, y: 0 },
    };
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            machines.push(machine.clone());
            continue;
        }
        let (ind, data) = line.split_once(':').unwrap();
        match ind {
            "Button A" => {
                let (x, y) = data.trim().split_once(',').unwrap();
                let x = x.trim().strip_prefix('X').unwrap().parse().unwrap();
                let y = y.trim().strip_prefix('Y').unwrap().parse().unwrap();
                machine.button_a = Offset { x, y };
            }
            "Button B" => {
                let (x, y) = data.trim().split_once(',').unwrap();
                let x = x.trim().strip_prefix('X').unwrap().parse().unwrap();
                let y = y.trim().strip_prefix('Y').unwrap().parse().unwrap();
                machine.button_b = Offset { x, y };
            }
            "Prize" => {
                let (x, y) = data.trim().split_once(',').unwrap();
                let x = x.trim().strip_prefix("X=").unwrap().parse().unwrap();
                let y = y.trim().strip_prefix("Y=").unwrap().parse().unwrap();
                machine.prize_position = Location { x, y };
            }
            _ => {}
        }
    }

    machines.push(machine);

    machines
}

// X = a.x * A + b.x * B
// Y = a.y * A + b.y * B
// A = (Y - b.y * B) / a.y
// B = (X - a.x * A) / b.x
// A = (Y - b.y * ((X - a.x * A) / b.x)) / a.y
// A * a.y = Y - b.y * (X - a.x * A) / b.x
// A * a.y = Y - b.y * X / b.x + b.y * a.x * A / b.x
// A * a.y - b.y * a.x * A / b.x = Y - b.y * X / b.x
// A * (a.y - b.y * a.x / b.x)  = Y - b.y * X / b.x
// A = (Y - b.y * X / b.x) / (a.y - b.y * a.x / b.x)

fn get_button_presses(machine: &Machine) -> Option<(usize, usize)> {
    let x = machine.prize_position.x as f64;
    let y = machine.prize_position.y as f64;
    let a_x = machine.button_a.x as f64;
    let a_y = machine.button_a.y as f64;
    let b_x = machine.button_b.x as f64;
    let b_y = machine.button_b.y as f64;

    let a_count = ((y - b_y * x / b_x) / (a_y - b_y * a_x / b_x)).round();
    let b_count = ((x - a_x * a_count) / b_x).round();
    if a_count.is_nan() || b_count.is_nan() {
        return None;
    }

    if (machine.button_a.x * a_count as isize + machine.button_b.x * b_count as isize) as usize
        == machine.prize_position.x
        && (machine.button_a.y * a_count as isize + machine.button_b.y * b_count as isize) as usize
            == machine.prize_position.y
    {
        Some((a_count as usize, b_count as usize))
    } else {
        None
    }
}

fn part1(buf: &mut dyn Read) {
    let machines = parse(buf);

    let result: usize = machines
        .iter()
        .filter_map(get_button_presses)
        .map(|(a, b)| a * 3 + b)
        .sum();

    println!("Part 1: {}", result);
}

fn part2(buf: &mut dyn Read) {
    let machines = parse(buf);

    let result: usize = machines
        .into_iter()
        .map(|machine| Machine {
            prize_position: Location {
                x: machine.prize_position.x + 10000000000000,
                y: machine.prize_position.y + 10000000000000,
            },
            ..machine
        })
        .filter_map(|machine| get_button_presses(&machine))
        .map(|(a, b)| a * 3 + b)
        .sum();

    println!("Part 1: {}", result);
}

inventory::submit!(Aoc::new(
    2024,
    13,
    part1,
    part2,
    include_bytes!("./inputs/day13")
));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_button_presses() {
        // Button A: X+94, Y+34
        // Button B: X+22, Y+67
        // Prize: X=8400, Y=5400
        assert_eq!(
            get_button_presses(&Machine {
                button_a: Offset { x: 94, y: 34 },
                button_b: Offset { x: 22, y: 67 },
                prize_position: Location { x: 8400, y: 5400 }
            }),
            Some((80, 40))
        );
        // Button A: X+26, Y+66
        // Button B: X+67, Y+21
        // Prize: X=12748, Y=12176
        assert_eq!(
            get_button_presses(&Machine {
                button_a: Offset { x: 26, y: 66 },
                button_b: Offset { x: 67, y: 21 },
                prize_position: Location { x: 12748, y: 12176 }
            }),
            None
        );
        // Button A: X+17, Y+86
        // Button B: X+84, Y+37
        // Prize: X=7870, Y=6450
        assert_eq!(
            get_button_presses(&Machine {
                button_a: Offset { x: 17, y: 86 },
                button_b: Offset { x: 84, y: 37 },
                prize_position: Location { x: 7870, y: 6450 }
            }),
            Some((38, 86))
        );
        // Button A: X+69, Y+23
        // Button B: X+27, Y+71
        // Prize: X=18641, Y=10279
        assert_eq!(
            get_button_presses(&Machine {
                button_a: Offset { x: 69, y: 23 },
                button_b: Offset { x: 27, y: 71 },
                prize_position: Location { x: 18641, y: 10279 }
            }),
            None
        );

        // assert_eq!(
        //     get_button_presses(&Machine {
        //         button_a: Offset { x: 2, y: 2 },
        //         button_b: Offset { x: 2, y: 2 },
        //         prize_position: Location { x: 20, y: 20 }
        //     }),
        //     Some((0, 10))
        // );

        // Button A: X+22, Y+61
        // Button B: X+45, Y+17
        // Prize: X=15827, Y=9764
        assert_eq!(
            get_button_presses(&Machine {
                button_a: Offset { x: 22, y: 61 },
                button_b: Offset { x: 45, y: 17 },
                prize_position: Location { x: 15827, y: 9764 }
            }),
            None,
        );

        assert_eq!(
            get_button_presses(&Machine {
                button_a: Offset { x: 42, y: 32 },
                button_b: Offset { x: 11, y: 86 },
                prize_position: Location { x: 2533, y: 8838 },
            },),
            Some((37, 89)),
        );
    }
}
