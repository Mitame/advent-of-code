use std::io::{BufRead, BufReader, Read};

use crate::Aoc;

type Number = num_bigint::BigUint;

#[derive(Debug)]
enum Operation {
    Add,
    Multiply,
}

impl Operation {
    fn perform(&self, a: Number, b: Number) -> Number {
        match self {
            Self::Add => a + b,
            Self::Multiply => a * b,
        }
    }
}

#[derive(Debug)]
struct Input {
    rows: Vec<Vec<Number>>,
    operations: Vec<Operation>,
}

fn parse(buf: &mut dyn Read) -> Input {
    let buf_reader = BufReader::new(buf);

    let mut operations: Option<Vec<Operation>> = None;
    let mut rows = vec![];
    for line in buf_reader.lines() {
        let line = line.unwrap();
        let cells = line.split(" ").filter(|c| !c.is_empty());
        let numbers: Result<Vec<Number>, _> = cells.clone().map(|v| v.trim().parse()).collect();
        if let Ok(numbers) = numbers {
            rows.push(numbers);
        } else {
            let operations_row: Result<Vec<Operation>, ()> = cells
                .map(|v| match v {
                    "+" => Ok(Operation::Add),
                    "*" => Ok(Operation::Multiply),
                    v => {
                        eprintln!("'{}'", v);
                        eprintln!("'{}'", line);
                        Err(())
                    }
                })
                .collect();
            operations = Some(operations_row.unwrap());
        }
    }

    Input {
        operations: operations.unwrap(),
        rows,
    }
}

fn part1(buf: &mut dyn Read) {
    let Input { rows, operations } = parse(buf);
    let mut solutions: Vec<Number> = vec![];
    for (i, operation) in operations.iter().enumerate() {
        let inputs = rows.iter().map(|row| &row[i]);
        let result = match operation {
            Operation::Add => inputs.sum(),
            Operation::Multiply => inputs.product(),
        };
        solutions.push(result);
    }
    let answer: Number = solutions.iter().sum();
    println!("Part 1: {}", answer);
}

fn part2(buf: &mut dyn Read) {
    let buf_reader = BufReader::new(buf);
    let mut lines = buf_reader.lines();
    let mut columns: Vec<String> = lines
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .map(|c| c.to_string())
        .collect();
    for line in lines {
        for (i, c) in line.unwrap().chars().enumerate() {
            columns[i].push(c);
        }
    }

    let mut operation: Option<Operation> = None;
    let mut values: Vec<Number> = vec![];
    let mut sum: Number = Default::default();
    for mut column in columns.iter().map(|column| column.trim()) {
        if column.is_empty() {
            {
                // Do calculation
                let operation = operation.unwrap();
                let value: Number = match operation {
                    Operation::Add => values.iter().sum(),
                    Operation::Multiply => values.iter().product(),
                };
                sum += value;
            }
            {
                // Cleanup
                values = vec![];
                operation = None;
            }
            continue;
        }

        let last_char = column.chars().last().unwrap();

        if !last_char.is_numeric() {
            operation = match last_char {
                '+' => Some(Operation::Add),
                '*' => Some(Operation::Multiply),
                _ => panic!(),
            };
            column = column[0..column.len() - 1].trim();
        }

        values.push(column.parse().unwrap())
    }

    let operation = operation.unwrap();
    let value: Number = match operation {
        Operation::Add => values.iter().sum(),
        Operation::Multiply => values.iter().product(),
    };
    sum += value;

    println!("Part 2: {}", sum);
}

inventory::submit!(Aoc::new(2025, 6, part1, part2,));
