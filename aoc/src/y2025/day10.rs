use std::io::{BufRead, BufReader, Read};

use good_lp::{
    Expression, ProblemVariables, Solution, SolverModel, Variable, constraint, default_solver,
    variable,
};

use crate::Aoc;

#[derive(Debug)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<u32>>,
    joltage_requirement: Vec<u32>,
}

fn parse_line(line: &str) -> Machine {
    let part_count = line.chars().filter(|c| c == &' ').count() + 1;
    let mut parts = line.split(" ");

    let lights: Vec<bool> = parts
        .next()
        .unwrap()
        .trim_matches(&['[', ']'])
        .chars()
        .map(|c| match c {
            '#' => true,
            '.' => false,
            _ => panic!("not a light: {}", c),
        })
        .collect();

    let buttons: Vec<Vec<u32>> = parts
        .clone()
        .take(part_count - 2)
        .map(|button| {
            button
                .trim_matches(&['(', ')'])
                .split(",")
                .map(|v| v.parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    let joltage_requirement: Vec<u32> = parts
        .last()
        .unwrap()
        .trim_matches(&['{', '}'])
        .split(",")
        .map(|v| v.parse::<u32>().unwrap())
        .collect();

    Machine {
        lights,
        buttons,
        joltage_requirement,
    }
}

fn parse(buf: &mut dyn Read) -> Vec<Machine> {
    let buf_reader = BufReader::new(buf);
    buf_reader
        .lines()
        .map(|line| parse_line(&line.unwrap()))
        .collect()
}

fn lights_to_mask(lights: &[bool]) -> u32 {
    lights.iter().enumerate().fold(0, |acc, (i, value)| {
        acc ^ value.then_some(1 << i).unwrap_or(0)
    })
}

fn button_to_mask(button: &[u32]) -> u32 {
    button
        .iter()
        .fold(0, |acc, light_index| acc ^ (1 << light_index))
}

fn binary_selector(i: u32, count: usize) -> impl Iterator<Item = usize> {
    (0..count).filter_map(move |b| (i & (1 << b) != 0).then_some(b as usize))
}

fn binary_select<'a, T>(i: u32, source: &'a [T]) -> Vec<&'a T> {
    binary_selector(i, source.len())
        .map(|j| &source[j])
        .collect()
}

fn button_combinations<'a>(buttons: &'a [Vec<u32>]) -> impl Iterator<Item = Vec<&'a Vec<u32>>> {
    (0..((2u32).pow(buttons.len() as u32))).map(|i| binary_select(i, buttons))
}

fn find_least_buttons_for_machine(machine: &Machine) -> usize {
    let target_mask = lights_to_mask(&machine.lights);
    button_combinations(&machine.buttons)
        .filter_map(|button_combination| {
            let button_count = button_combination.len();
            let result = button_combination
                .iter()
                .map(|button| button_to_mask(&button))
                .fold(0, |acc, mask| acc ^ mask);

            (result == target_mask).then_some(button_count)
        })
        .min()
        .unwrap()
}

fn part1(buf: &mut dyn Read) {
    let machines = parse(buf);
    let result: usize = machines
        .iter()
        .map(|machine| find_least_buttons_for_machine(machine))
        .sum();
    println!("Part 1: {}", result);
}

fn find_least_buttons_for_machine_joltage(machine: &Machine) -> u32 {
    // Let's just solve it with Linear Programming
    let mut problem_variables = ProblemVariables::new();
    let variables: Vec<Variable> = machine
        .buttons
        .iter()
        .map(|_| problem_variables.add(variable().integer().min(0)))
        .collect();

    let problem = problem_variables
        .minimise(
            variables
                .iter()
                .copied()
                .fold(Expression::default(), |acc, next| acc + next),
        )
        .using(default_solver)
        .with_all(
            machine
                .joltage_requirement
                .iter()
                .enumerate()
                .map(|(i, joltage)| {
                    let mut expression = Expression::default();
                    for (button, variable) in machine.buttons.iter().zip(variables.iter()) {
                        if button.contains(&(i as u32)) {
                            expression += variable
                        }
                    }
                    constraint!(expression == *joltage)
                }),
        );

    let solution = problem.solve().unwrap();

    let presses: f64 = variables.iter().map(|var| solution.value(*var)).sum();
    dbg!(presses, presses as u32);
    presses as u32
}

fn part2(buf: &mut dyn Read) {
    let machines = parse(buf);
    let result: u32 = machines
        .iter()
        .map(|machine| find_least_buttons_for_machine_joltage(machine))
        .sum();
    println!("Part 2: {}", result);
}

inventory::submit!(Aoc::new(2025, 10, part1, part2,));

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_least_buttons_for_machine_joltage_ex1() {
        // [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
        let machine = Machine {
            lights: vec![],
            buttons: vec![
                vec![3],
                vec![1, 3],
                vec![2],
                vec![2, 3],
                vec![0, 2],
                vec![0, 1],
            ],
            joltage_requirement: vec![3, 4, 3, 7],
        };

        assert_eq!(find_least_buttons_for_machine_joltage(&machine), 10);
    }

    #[test]
    fn test_find_least_buttons_for_machine_joltage_ex2() {
        // [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
        let machine = Machine {
            lights: vec![],
            buttons: vec![
                vec![0, 2, 3, 4],
                vec![2, 3],
                vec![0, 4],
                vec![0, 1, 2],
                vec![1, 2, 3, 4],
            ],
            joltage_requirement: vec![7, 5, 12, 7, 2],
        };

        assert_eq!(find_least_buttons_for_machine_joltage(&machine), 12);
    }

    #[test]
    fn test_find_least_buttons_for_machine_joltage_ex3() {
        // [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
        let machine = Machine {
            lights: vec![],
            buttons: vec![
                vec![0, 1, 2, 3, 4],
                vec![0, 3, 4],
                vec![0, 1, 2, 4, 5],
                vec![1, 2],
            ],
            joltage_requirement: vec![10, 11, 11, 5, 10, 5],
        };

        assert_eq!(find_least_buttons_for_machine_joltage(&machine), 11);
    }

    #[test]
    fn test_find_least_buttons_for_machine_joltage_l1() {
        // [###.#.] (1,3,5) (0,2,5) (1,3) (2,3,5) (0,2,4) (0,1,2,4) {44,25,63,29,33,40}
        let machine = Machine {
            lights: vec![],
            buttons: vec![
                vec![1, 3, 5],
                vec![0, 2, 5],
                vec![1, 3],
                vec![2, 3, 5],
                vec![0, 2, 4],
                vec![0, 1, 2, 4],
            ],
            joltage_requirement: vec![44, 25, 63, 29, 33, 40],
        };

        // 11, 0,2,5
        // 4,  2,3,5
        // 10, 1,3,5
        // 3,  0,2,4
        // 15, 0,1,2,4
        // 15, 0,2,4
        // 15, 2,3,5

        // 11, 0,2,5
        // 10, 1,3,5
        // 15, 0,1,2,4
        // 18, 0,2,4
        // 19, 2,3,5

        assert_eq!(find_least_buttons_for_machine_joltage(&machine), 73);
    }
}
