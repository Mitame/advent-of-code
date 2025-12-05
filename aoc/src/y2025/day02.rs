use std::{
    io::{BufRead, BufReader, Read},
    ops::RangeInclusive,
};

use crate::Aoc;

fn parse(buf: &mut dyn Read) -> Vec<RangeInclusive<usize>> {
    let mut buf_read = BufReader::new(buf);
    let mut line = String::new();
    buf_read.read_line(&mut line).unwrap();
    line.split(",")
        .map(|range_str| {
            let (a, b) = range_str.split_once("-").unwrap();
            let a: usize = a.trim().parse().unwrap();
            let b: usize = b.trim().parse().unwrap();
            a..=b
        })
        .collect()
}

fn id_has_repeats(id: usize, repeats: u32) -> bool {
    let digits = id.ilog10() + 1;
    if digits.rem_euclid(repeats) != 0 {
        return false;
    }

    let mask = 10_usize.pow(digits / repeats);
    let lower_id = id % mask;
    (1..repeats).all(|i| {
        let id_part = (id / mask.pow(i)) % mask;
        id_part == lower_id
    })
}

fn is_invalid_id(id: usize) -> bool {
    let digits = id.ilog10() + 1;
    (2..=digits).any(|repeats| id_has_repeats(id, repeats))
}

fn part1(buf: &mut dyn Read) {
    let id_ranges = parse(buf);
    let answer: usize = id_ranges
        .into_iter()
        .flatten()
        .filter(|id| id_has_repeats(*id, 2))
        .sum();
    println!("Part 1: {}", answer);
}

fn part2(buf: &mut dyn Read) {
    let id_ranges = parse(buf);
    let answer: usize = id_ranges
        .into_iter()
        .flatten()
        .filter(|id| is_invalid_id(*id))
        .sum();
    println!("Part 2: {}", answer);
}

inventory::submit!(Aoc::new(2025, 2, part1, part2,));

#[cfg(test)]
mod tests {
    use crate::y2025::day02::{id_has_repeats, is_invalid_id};

    #[test]
    fn test_id_has_repeats() {
        assert_eq!(id_has_repeats(11, 2), true);
        assert_eq!(id_has_repeats(22, 2), true);
        assert_eq!(id_has_repeats(99, 2), true);
        assert_eq!(id_has_repeats(1010, 2), true);
        assert_eq!(id_has_repeats(1188511885, 2), true);
        assert_eq!(id_has_repeats(222222, 2), true);
        assert_eq!(id_has_repeats(446446, 2), true);
        assert_eq!(id_has_repeats(38593859, 2), true);

        assert_eq!(id_has_repeats(123123, 2), true);
        assert_eq!(id_has_repeats(35535, 2), false);
        assert_eq!(id_has_repeats(123124, 2), false);
        assert_eq!(id_has_repeats(101, 2), false);
    }

    #[test]
    fn test_is_invalid_id() {
        assert_eq!(is_invalid_id(11), true);
        assert_eq!(is_invalid_id(22), true);
        assert_eq!(is_invalid_id(99), true);
        assert_eq!(is_invalid_id(111), true);
        assert_eq!(is_invalid_id(999), true);
        assert_eq!(is_invalid_id(1010), true);
        assert_eq!(is_invalid_id(1188511885), true);
        assert_eq!(is_invalid_id(222222), true);
        assert_eq!(is_invalid_id(446446), true);
        assert_eq!(is_invalid_id(565656), true);
        assert_eq!(is_invalid_id(824824824), true);
        assert_eq!(is_invalid_id(2121212121), true);
        assert_eq!(is_invalid_id(38593859), true);
    }
}
