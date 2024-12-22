use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
};

use crate::Aoc;

struct SecretGenerator {
    state: u32,
}

impl SecretGenerator {
    fn new(seed: u32) -> SecretGenerator {
        SecretGenerator { state: seed }
    }

    fn generate(&mut self) -> u32 {
        // Defo simplifyable
        // 16777216 is 0x1000000
        let next = self.state;
        let next = ((next << 6) ^ next) & 0xffffff;
        let next = ((next >> 5) ^ next) & 0xffffff;
        let next = ((next << 11) ^ next) & 0xffffff;
        self.state = next;
        next
    }
}

impl Iterator for SecretGenerator {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.generate())
    }
}

fn parse(buf: &mut dyn Read) -> Vec<u32> {
    let reader = BufReader::new(buf);
    reader
        .lines()
        .map_while(|line| line.ok()?.parse().ok())
        .collect()
}

fn part1(buf: &mut dyn Read) {
    let seeds = parse(buf);
    let result: usize = seeds
        .into_iter()
        .map(|seed| SecretGenerator::new(seed).nth(1999).unwrap() as usize)
        .sum();
    println!("Part 1: {}", result);
}

fn part2(buf: &mut dyn Read) {
    let seeds = parse(buf);
    let monkey_prices = seeds.into_iter().map(|seed| {
        SecretGenerator::new(seed)
            .take(2000)
            .map(|price| (price % 10) as i8)
            .collect::<Vec<_>>()
    });

    let mut change_sequence_score: HashMap<[i8; 4], i16> = HashMap::new();
    for prices in monkey_prices {
        // Calculate the price changes over time
        let change: Vec<i8> = prices.windows(2).map(|vals| vals[1] - vals[0]).collect();

        // Create a map from change sequence to the price at that time
        let monkey_change_sequence_score: HashMap<[i8; 4], i8> = change
            .windows(4)
            .zip(prices.iter().skip(4))
            .map(|(changes, value)| {
                let changes: [i8; 4] = changes.try_into().unwrap();
                (changes, *value)
            })
            .fold(HashMap::new(), |mut acc, (changes, value)| {
                acc.entry(changes).or_insert(value);
                acc
            });

        // Update the scores for each sequence
        for (changes, value) in monkey_change_sequence_score.into_iter() {
            *change_sequence_score.entry(changes).or_default() += value as i16;
        }
    }

    // Find the maximum score (the sequence doesn't matter)
    let result = change_sequence_score.values().max().unwrap();

    println!("Part 2: {}", result);
}

inventory::submit!(Aoc::new(
    2024,
    22,
    part1,
    part2,
    include_bytes!("./inputs/day22")
));

#[cfg(test)]
mod tests {
    use super::SecretGenerator;

    #[test]
    fn secret_generator() {
        let gen = SecretGenerator::new(123);
        let numbers: Vec<_> = gen.take(10).collect();
        assert_eq!(
            numbers,
            vec![
                15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
                5908254,
            ]
        )
    }
}
