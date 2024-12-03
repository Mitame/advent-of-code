use scan_fmt::{scan_fmt, scanln_fmt};
use std::collections::HashMap;

fn part1(mut list_a: Vec<u32>, mut list_b: Vec<u32>) {
    list_a.sort();
    list_b.sort();

    let result: u32 = list_a
        .into_iter()
        .zip(list_b)
        .map(|(a, b)| a.abs_diff(b))
        .sum();
    println!("Part 1: {}", result);
}

fn count(list: Vec<u32>) -> HashMap<u32, u32> {
    let mut counter = HashMap::new();

    for item in list {
        *counter.entry(item).or_default() += 1;
    }

    counter
}

fn part2(list_a: Vec<u32>, list_b: Vec<u32>) {
    let count_a = count(list_a);
    let count_b = count(list_b);

    let score: u32 = count_a
        .into_iter()
        .map(|(n, count)| n * count * count_b.get(&n).cloned().unwrap_or(0))
        .sum();
    println!("Part 2: {}", score);
}

fn main() {
    let mut list_a = Vec::<u32>::new();
    let mut list_b = Vec::<u32>::new();

    loop {
        let res = scanln_fmt!("{d} {d}", u32, u32);
        match res {
            Ok((a, b)) => {
                list_a.push(a);
                list_b.push(b);
            }
            Err(_) => {
                break;
            }
        }
    }

    part1(list_a.clone(), list_b.clone());

    part2(list_a, list_b);
}
