use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
};

use regex::Regex;

use crate::Aoc;

struct Data {
    towels: Vec<String>,
    patterns: Vec<String>,
}

fn parse(buf: &mut dyn Read) -> Data {
    let reader = BufReader::new(buf);
    let mut lines = reader.lines();
    let towels: Vec<_> = lines
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(str::trim)
        .map(|v| v.to_owned())
        .collect();
    let patterns: Vec<_> = lines.skip(1).map_while(Result::ok).collect();

    Data { towels, patterns }
}

fn part1(buf: &mut dyn Read) {
    let Data { towels, patterns } = parse(buf);
    let metapattern = Regex::new(&format!("^({})+$", towels.join("|"))).unwrap();
    let result = patterns
        .into_iter()
        .filter(|pattern| metapattern.is_match(pattern))
        .count();

    println!("Part 1: {}", result);
}

fn count_options(
    pattern: &str,
    towels_by_size: &[Vec<&str>],
    cache: &mut HashMap<String, usize>,
) -> usize {
    if pattern.is_empty() {
        return 1;
    }

    let search_max = (pattern.len() + 1).min(towels_by_size.len());

    towels_by_size[..search_max]
        .iter()
        .map(|towels| {
            if let Some(remaining) = towels
                .iter()
                .filter_map(|towel| pattern.strip_prefix(*towel))
                .next()
            {
                if let Some(value) = cache.get(remaining) {
                    *value
                } else {
                    let value = count_options(remaining, towels_by_size, cache);
                    cache.insert(remaining.to_string(), value);
                    value
                }
            } else {
                0
            }
        })
        .sum()
}

fn part2(buf: &mut dyn Read) {
    let Data { towels, patterns } = parse(buf);
    let metapattern = Regex::new(&format!("^({})+$", towels.join("|"))).unwrap();
    let valid_patterns: Vec<_> = patterns
        .into_iter()
        .filter(|pattern| metapattern.is_match(pattern))
        .collect();

    let max_towel_size = towels.iter().map(|t| t.len()).max().unwrap();
    let mut towels_by_size: Vec<Vec<&str>> = vec![vec![]; max_towel_size + 1];
    towels.iter().for_each(|t| towels_by_size[t.len()].push(t));

    let result: usize = valid_patterns
        .iter()
        .map(|pattern| count_options(pattern, &towels_by_size, &mut HashMap::new()))
        .sum();

    println!("Part 2: {}", result);
}

inventory::submit!(Aoc::new(
    2024,
    19,
    part1,
    part2,
    include_bytes!("./inputs/day19")
));
