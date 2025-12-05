use std::io::{BufRead, BufReader, Read};

use regex::Regex;

use crate::Aoc;

fn part1(buf: &mut dyn Read) {
    let reader = BufReader::new(buf);
    let result: u32 = reader
        .lines()
        .map_while(|line| {
            let Ok(line) = line else {
                return None;
            };
            let first_digit = line.chars().find(|c| c.is_numeric()).unwrap();
            let last_digit = line.chars().filter(|c| c.is_numeric()).next_back().unwrap();
            Some(
                format!("{}{}", first_digit, last_digit)
                    .parse::<u32>()
                    .unwrap(),
            )
        })
        .sum();

    println!("Part 1: {}", result);
}

fn convert_text(text: &str) -> &str {
    match text {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        d => d,
    }
}

fn part2(buf: &mut dyn Read) {
    let reader = BufReader::new(buf);

    // The line may read something like `123oneight`, which has an overlapping one and eight.
    // To handle this, we use a regex to preferentially select the /last/ item in the overlap.
    let re = Regex::new(r"^.*?(\d|one|two|three|four|five|six|seven|eight|nine).*(\d|one|two|three|four|five|six|seven|eight|nine).*?$|^.*?(\d|one|two|three|four|five|six|seven|eight|nine).*?$").unwrap();
    let result: u32 = reader
        .lines()
        .map_while(|line| {
            let Ok(line) = line else {
                return None;
            };

            let capture = re.captures(&line).unwrap();
            let first_digit = capture.get(1).or_else(|| capture.get(3)).unwrap().as_str();
            let last_digit = capture.get(2).or_else(|| capture.get(3)).unwrap().as_str();
            let first_digit = convert_text(first_digit);
            let last_digit = convert_text(last_digit);
            Some(
                format!("{}{}", first_digit, last_digit)
                    .parse::<u32>()
                    .unwrap(),
            )
        })
        .sum();

    println!("Part 2: {}", result);
}

inventory::submit!(Aoc::new(2023, 1, part1, part2,));
