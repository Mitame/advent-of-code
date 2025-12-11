use std::{
    collections::{HashMap, HashSet},
    io::{BufRead, BufReader, Read},
};

use crate::Aoc;

fn parse(buf: &mut dyn Read) -> HashMap<String, HashSet<String>> {
    let buf_reader = BufReader::new(buf);

    buf_reader
        .lines()
        .map(|line| {
            let line = line.unwrap();

            let (from_label, to_labels) = line.split_once(':').unwrap();
            let to_labels: HashSet<String> = to_labels
                .split(' ')
                .map(|label| label.trim().to_string())
                .collect();
            (from_label.to_string(), to_labels)
        })
        .collect()
}

fn count_routes_between(edges: &HashMap<String, HashSet<String>>, a: &str, b: &str) -> usize {
    let mut route_counts: HashMap<&str, usize> = HashMap::from([(a, 1)]);

    let mut active: HashMap<&str, usize> = HashMap::from([(a, 1)]);
    let empty = HashSet::default();
    while !active.is_empty() {
        let new = active
            .into_iter()
            .map(|(label, count)| (edges.get(label).unwrap_or(&empty), count))
            .fold(HashMap::new(), |mut acc, (labels, count)| {
                for label in labels {
                    *acc.entry(label.as_str()).or_default() += count;
                }
                acc
            });
        for (label, routes) in new.iter() {
            *route_counts.entry(*label).or_default() += routes;
        }
        active = new;
    }

    route_counts.get(b).copied().unwrap_or_default()
}

fn part1(buf: &mut dyn Read) {
    let edges = parse(buf);
    let answer = count_routes_between(&edges, "you", "out");

    println!("Part 1: {}", answer);
}

fn part2(buf: &mut dyn Read) {
    let edges = parse(buf);

    let svr_fft_routes = count_routes_between(&edges, "svr", "fft");
    let fft_dac_routes = count_routes_between(&edges, "fft", "dac");
    let dac_out_routes = count_routes_between(&edges, "dac", "out");

    println!(
        "Part 2: {}",
        svr_fft_routes * fft_dac_routes * dac_out_routes
    );
}

inventory::submit!(Aoc::new(2025, 11, part1, part2,));
