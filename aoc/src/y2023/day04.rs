use std::{
    collections::{HashMap, HashSet},
    io::{BufRead, BufReader, Read},
};

use crate::Aoc;

#[derive(Debug, Hash, PartialEq, Eq)]
struct Card {
    id: usize,
    numbers: Vec<usize>,
    winning_numbers: Vec<usize>,
}

fn parse(buf: &mut dyn Read) -> Vec<Card> {
    let reader = BufReader::new(buf);
    reader
        .lines()
        .map_while(|line| {
            let line = line.ok()?;
            let (label, numbers) = line.split_once(':')?;
            let id: usize = label.trim().split_once(' ')?.1.trim().parse().ok()?;
            let (numbers, winning_numbers) = numbers.split_once('|')?;
            let numbers: Vec<usize> = numbers
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|v| v.parse().unwrap())
                .collect();
            let winning_numbers: Vec<usize> = winning_numbers
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|v| v.parse().unwrap())
                .collect();
            Some(Card {
                id,
                numbers,
                winning_numbers,
            })
        })
        .collect()
}

fn part1(buf: &mut dyn Read) {
    let cards = parse(buf);

    let result: usize = cards
        .into_iter()
        .map(|card| {
            dbg!(&card);
            let numbers: HashSet<_> = card.numbers.into_iter().collect();
            let winning_numbers: HashSet<_> = card.winning_numbers.into_iter().collect();
            let count = numbers.intersection(&winning_numbers).count();
            if count == 0 {
                0
            } else {
                2usize.pow(count as u32 - 1)
            }
        })
        .sum();

    println!("Part 1: {}", result);
}

fn part2(buf: &mut dyn Read) {
    let cards = parse(buf);
    let mut won_cards: HashMap<usize, usize> = cards
        .iter()
        .enumerate()
        .map(|(index, _)| (index, 1))
        .collect();

    for (i, card) in cards.into_iter().enumerate() {
        let numbers: HashSet<_> = card.numbers.into_iter().collect();
        let winning_numbers: HashSet<_> = card.winning_numbers.into_iter().collect();
        let count = numbers.intersection(&winning_numbers).count();
        let score = *won_cards.get(&i).unwrap();
        for j in 0..count {
            *won_cards.entry(i + j + 1).or_default() += score;
        }
    }

    let result: usize = won_cards.values().sum();

    println!("Part 2: {}", result);
}

inventory::submit!(Aoc::new(
    2023,
    4,
    part1,
    part2,
    include_bytes!("./inputs/day04")
));
