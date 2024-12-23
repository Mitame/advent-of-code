use std::{
    collections::{HashMap, HashSet},
    io::{BufRead, BufReader, Read},
};

use crate::Aoc;

fn parse(buf: &mut dyn Read) -> Vec<(String, String)> {
    let reader = BufReader::new(buf);
    reader
        .lines()
        .map_while(|line| {
            line.ok()?
                .split_once('-')
                .and_then(|(a, b)| Some((a.parse().ok()?, b.parse().ok()?)))
        })
        .collect()
}

fn part1(buf: &mut dyn Read) {
    let pairs = parse(buf);
    let connections: HashMap<_, HashSet<String>> =
        pairs.into_iter().fold(HashMap::new(), |mut acc, (a, b)| {
            acc.entry(a.clone()).or_default().insert(b.clone());
            acc.entry(b).or_default().insert(a);
            acc
        });
    let mut networks: HashSet<[&String; 3]> = HashSet::new();
    for (a, bs) in &connections {
        if a.starts_with('t') {
            for b in bs {
                for c in connections.get(b).into_iter().flatten() {
                    let is_loop = connections.get(c).map(|ds| ds.contains(a)).unwrap_or(false);
                    if is_loop {
                        let mut network = [a, b, c];
                        network.sort();
                        networks.insert(network);
                    }
                }
            }
        }
    }

    let result = networks.len();
    println!("Part 1: {}", result);
}

fn part2(buf: &mut dyn Read) {
    let pairs = parse(buf);
    let connections: HashMap<_, HashSet<String>> =
        pairs.into_iter().fold(HashMap::new(), |mut acc, (a, b)| {
            acc.entry(a.clone()).or_default().insert(b.clone());
            acc.entry(b.clone()).or_default().insert(a.clone());
            acc
        });

    let mut network_map: HashMap<String, HashSet<_>> = HashMap::new();
    for node in connections.keys() {
        let network = [node.clone()].into();
        network_map.insert(node.clone(), network);
    }

    for (node, connections) in connections {
        for connecting_node in connections.iter() {
            let connecting_network = network_map.get_mut(connecting_node).unwrap();
            if connecting_network.intersection(&connections).count() == connecting_network.len() {
                connecting_network.insert(node.clone());
            }
        }
    }

    let mut complete_networks: Vec<_> = network_map.values().collect();
    complete_networks.sort_by_key(|network| network.len());
    let mut largest_network: Vec<_> = complete_networks.last().unwrap().iter().cloned().collect();
    largest_network.sort();

    let result = largest_network.join(",");

    println!("Part 2: {}", result);
}

inventory::submit!(Aoc::new(
    2024,
    23,
    part1,
    part2,
    include_bytes!("./inputs/day23")
));
