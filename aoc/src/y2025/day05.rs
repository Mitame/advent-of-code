use std::io::{BufRead, BufReader, Read};
use std::ops::RangeInclusive;

use crate::Aoc;

type Ingredient = u64;

fn parse(buf: &mut dyn Read) -> (Vec<RangeInclusive<Ingredient>>, Vec<Ingredient>) {
    let buf_reader = BufReader::new(buf);
    let mut lines = buf_reader.lines().into_iter();

    let mut ranges = vec![];
    loop {
        let range_line = lines.next().unwrap().unwrap();
        if range_line.is_empty() {
            break;
        }
        let (a, b) = range_line.split_once('-').unwrap();
        let a: Ingredient = a.parse().unwrap();
        let b: Ingredient = b.parse().unwrap();

        ranges.push(a..=b);
    }

    let values: Vec<Ingredient> = lines.map(|v| v.unwrap().parse().unwrap()).collect();
    (ranges, values)
}

fn part1(buf: &mut dyn Read) {
    let (fresh_ranges, ingredients) = parse(buf);

    let fresh_ingredients_count = ingredients
        .iter()
        .filter(|ingredient| fresh_ranges.iter().any(|r| r.contains(ingredient)))
        .count();
    println!("Part 1: {}", fresh_ingredients_count);
}

fn part2(buf: &mut dyn Read) {
    let (fresh_ranges, ingredients) = parse(buf);

    let mut markers: Vec<(Ingredient, bool)> = vec![];

    for range in fresh_ranges {
        markers.push((*range.start(), true));
        markers.push((*range.end(), false));
    }

    markers.sort_by_key(|(k, _)| *k);
    let mut start_ingredients = vec![];
    let mut last_end = None;
    let mut possible_fresh_ingredient_count = 0;
    for (id, is_start) in markers {
        if is_start {
            start_ingredients.push(id);
            if Some(id) == last_end {
                possible_fresh_ingredient_count -= 1;
            }
        } else {
            let start_ingredient = start_ingredients.pop().unwrap();
            if start_ingredients.is_empty() {
                last_end = Some(id);
                possible_fresh_ingredient_count += id - start_ingredient + 1;
            }
        }
    }
    println!("Part 2: {}", possible_fresh_ingredient_count);
}

inventory::submit!(Aoc::new(2025, 5, part1, part2,));
