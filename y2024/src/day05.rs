use crate::Aoc;
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    io::{BufRead, BufReader, Read},
};

type Rules = HashMap<usize, HashSet<usize>>;

type Pages = Vec<usize>;

struct Data {
    rules: Rules,
    updates: Vec<Pages>,
}

fn parse(buf: &mut dyn Read) -> Data {
    let reader = BufReader::new(buf);
    let mut rules: Rules = HashMap::new();
    let mut updates: Vec<Pages> = Vec::new();

    let mut is_pages = false;
    for line in reader.lines() {
        let line = line.unwrap();
        if line == "" {
            is_pages = true;
            continue;
        }

        if !is_pages {
            let numbers: Vec<usize> = line
                .split("|")
                .map(|v| v.parse::<usize>().unwrap())
                .collect();
            let before = numbers[0];
            let after = numbers[1];
            rules.entry(after).or_default().insert(before);
        } else {
            let numbers = line
                .split(",")
                .map(|v| v.parse::<usize>().unwrap())
                .collect();
            updates.push(numbers);
        }
    }

    Data { rules, updates }
}

fn is_valid(rules: &Rules, update: &[usize]) -> bool {
    let all_pages: HashSet<usize> = HashSet::from_iter(update.iter().cloned());
    let mut seen_pages = HashSet::new();
    let default = HashSet::new();
    for page in update.iter() {
        let rule_pages = rules.get(page).unwrap_or(&default);
        let filtered_pages: HashSet<usize> = rule_pages.intersection(&all_pages).cloned().collect();
        if filtered_pages.difference(&seen_pages).next().is_some() {
            return false;
        }
        seen_pages.insert(*page);
    }

    return true;
}

fn get_middle_page(update: &[usize]) -> usize {
    return update[update.len() / 2];
}

fn part1(buf: &mut dyn Read) {
    let Data { rules, updates } = parse(buf);
    let result: usize = updates
        .iter()
        .filter(|update| is_valid(&rules, update))
        .map(|update| get_middle_page(update))
        .sum();
    println!("Part 1: {}", result);
}
inventory::submit!(Aoc::new(5, 1, part1));

fn reorder_pages(rules: &Rules, rules_rev: &Rules, update: &[usize]) -> Vec<usize> {
    let mut update = update.to_vec();
    update.sort_by(|a, b| {
        if rules
            .get(a)
            .map(|befores| befores.contains(b))
            .unwrap_or(false)
        {
            Ordering::Greater
        } else if rules_rev
            .get(a)
            .map(|afters| afters.contains(b))
            .unwrap_or(false)
        {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    });
    update
}

fn part2(buf: &mut dyn Read) {
    let Data { rules, updates } = parse(buf);
    let mut rules_rev: Rules = HashMap::new();
    for (after, befores) in &rules {
        for before in befores {
            rules_rev.entry(*before).or_default().insert(*after);
        }
    }
    let bad_updates: Vec<&Pages> = updates
        .iter()
        .filter(|update| !is_valid(&rules, update))
        .collect();
    let result: usize = bad_updates
        .into_iter()
        .map(|update| reorder_pages(&rules, &rules_rev, update))
        .map(|update| get_middle_page(&update))
        .sum();

    println!("Part 2: {}", result);
}
inventory::submit!(Aoc::new(5, 2, part2));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        let mut rules = HashMap::new();
        rules.insert(1, HashSet::from([2, 3, 4]));
        assert!(is_valid(&rules, &[2, 4, 3, 1]));
        assert!(!is_valid(&rules, &[2, 4, 1, 3]));
    }

    #[test]
    fn test_reorder_pages() {
        let mut rules = HashMap::new();
        rules.insert(1, HashSet::from([2, 3, 4]));
        let mut rules_rev: Rules = HashMap::new();
        for (after, befores) in &rules {
            for before in befores {
                rules_rev.entry(*before).or_default().insert(*after);
            }
        }
        assert_eq!(
            reorder_pages(&rules, &rules_rev, &[2, 4, 3, 1]),
            vec![2, 4, 3, 1]
        );
        assert_eq!(
            reorder_pages(&rules, &rules_rev, &[2, 4, 1, 3]),
            vec![2, 4, 3, 1]
        );
    }
}
