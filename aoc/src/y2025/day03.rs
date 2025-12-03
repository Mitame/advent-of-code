use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
};

use crate::Aoc;

fn parse(buf: &mut dyn Read) -> Vec<String> {
    let buf_reader = BufReader::new(buf);
    let lines: Result<_, _> = buf_reader.lines().collect();
    lines.unwrap()
}

fn find_highest_joltage_2(bank: &str) -> u8 {
    let highest_positions = bank
        .chars()
        .enumerate()
        .fold(HashMap::new(), |mut acc, (i, char)| {
            acc.entry(char).or_insert(i);
            acc
        });
    let lowest_positions = bank
        .chars()
        .enumerate()
        .fold(HashMap::new(), |mut acc, (i, char)| {
            acc.insert(char, i);
            acc
        });
    for a in "9876543210".chars() {
        for b in "9876543210".chars() {
            let Some(a_pos) = highest_positions.get(&a) else {
                continue;
            };
            let Some(b_pos) = lowest_positions.get(&b) else {
                continue;
            };

            if a_pos < b_pos {
                return format!("{a}{b}").parse().unwrap();
            }
        }
    }

    // This can only happen if the bank has a length less than 2
    0
}

fn part1(buf: &mut dyn Read) {
    let banks = parse(buf);
    let result: u32 = banks
        .iter()
        .map(|bank| find_highest_joltage_2(bank) as u32)
        .sum();
    println!("Part 1: {result}");
}

fn find_highest_joltage_n(bank: &str, n: usize) -> Option<u64> {
    if n > bank.len() {
        return None;
    }
    let highest_positions = bank
        .chars()
        .enumerate()
        .fold(HashMap::new(), |mut acc, (i, char)| {
            acc.entry(char).or_insert(i);
            acc
        });

    for a in "9876543210".chars() {
        let Some(a_pos) = highest_positions.get(&a) else {
            continue;
        };
        if n == 1 {
            return Some(a.to_digit(10).unwrap() as u64);
        }
        let highest_subjoltage = find_highest_joltage_n(&bank[(a_pos + 1)..], n - 1);
        if let Some(highest_subjoltage) = highest_subjoltage {
            return Some(format!("{a}{highest_subjoltage}").parse().unwrap());
        }
    }

    return None;
}

fn part2(buf: &mut dyn Read) {
    let banks = parse(buf);
    let result: u64 = banks
        .iter()
        .map(|bank| find_highest_joltage_n(bank, 12).unwrap())
        .sum();
    println!("Part 2: {result}");
}

inventory::submit!(Aoc::new(
    2025,
    3,
    part1,
    part2,
    include_bytes!("./inputs/day03")
));

#[cfg(test)]
mod tests {
    use crate::y2025::day03::{find_highest_joltage_2, find_highest_joltage_n};

    #[test]
    fn test_find_higest_joltage_2() {
        assert_eq!(find_highest_joltage_2("987654321111111"), 98);
        assert_eq!(find_highest_joltage_2("811111111111119"), 89);
        assert_eq!(find_highest_joltage_2("234234234234278"), 78);
        assert_eq!(find_highest_joltage_2("818181911112111"), 92);
    }

    #[test]
    fn test_find_higest_joltage_12() {
        assert_eq!(find_highest_joltage_n("987654321111111", 2), Some(98));
        assert_eq!(find_highest_joltage_n("811111111111119", 2), Some(89));
        assert_eq!(find_highest_joltage_n("234234234234278", 2), Some(78));
        assert_eq!(find_highest_joltage_n("818181911112111", 2), Some(92));
        assert_eq!(
            find_highest_joltage_n("987654321111111", 12),
            Some(987654321111)
        );
        assert_eq!(
            find_highest_joltage_n("811111111111119", 12),
            Some(811111111119)
        );
        assert_eq!(
            find_highest_joltage_n("234234234234278", 12),
            Some(434234234278)
        );
        assert_eq!(
            find_highest_joltage_n("818181911112111", 12),
            Some(888911112111)
        );
    }
}
