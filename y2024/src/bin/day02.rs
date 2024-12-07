use std::io::{stdin, BufRead, BufReader};

type Levels = Vec<u8>;
type Report = Vec<Levels>;

fn main() {
    let reader = BufReader::new(stdin());

    let mut report: Report = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();

        report.push(line.split(' ').map(|v| v.parse().unwrap()).collect())
    }

    part1(report.clone());
    part2(report.clone());
}

fn is_safe(levels: Levels) -> bool {
    let mut is_increasing = true;
    let mut is_decreasing = true;
    let mut previous_level_opt: Option<u8> = None;

    for level in levels {
        // dbg!(is_increasing, is_decreasing, previous_level_opt, level);

        if previous_level_opt.is_none() {
            previous_level_opt = Some(level);
            continue;
        }

        let previous_level = previous_level_opt.unwrap();

        let bad;
        if previous_level < level && is_increasing {
            is_increasing = true;
            is_decreasing = false;

            let diff = level - previous_level;
            bad = diff == 0 || diff > 3;
        } else if previous_level > level && is_decreasing {
            is_decreasing = true;
            is_increasing = false;

            let diff = previous_level - level;
            bad = diff == 0 || diff > 3;
        } else {
            bad = true;
        }

        if bad {
            return false;
        } else {
            previous_level_opt = Some(level);
            continue;
        }
    }

    true
}

fn part1(report: Report) {
    let safe_count = report.into_iter().map(is_safe).filter(|v| *v).count();

    println!("Part 1: {}", safe_count);
}

fn is_safeish(levels: Levels) -> bool {
    for skip_index in 0..levels.len() {
        let new_levels: Levels = levels
            .clone()
            .into_iter()
            .enumerate()
            .filter_map(|(i, v)| (i != skip_index).then_some(v))
            .collect();
        if is_safe(new_levels.clone()) {
            return true;
        }
    }

    false
}

fn part2(report: Report) {
    let safeish_count = report
        .into_iter()
        .map(|levels| is_safeish(levels.clone()))
        .filter(|v| *v)
        .count();

    println!("Part 2: {}", safeish_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe() {
        assert!(is_safe(vec![7, 6, 4, 2, 1]));
        assert!(!is_safe(vec![1, 2, 7, 8, 9]));
        assert!(!is_safe(vec![9, 7, 2, 6, 1]));
        assert!(!is_safe(vec![1, 3, 2, 4, 5]));
        assert!(!is_safe(vec![8, 6, 4, 4, 1]));
        assert!(is_safe(vec![1, 3, 6, 7, 9]));
    }

    #[test]
    fn test_safeish() {
        assert!(is_safeish(vec![7, 6, 4, 2, 1]));
        assert!(!is_safeish(vec![1, 2, 7, 8, 9]));
        assert!(!is_safeish(vec![9, 7, 2, 6, 1]));
        assert!(is_safeish(vec![1, 3, 2, 4, 5]));
        assert!(is_safeish(vec![8, 6, 4, 4, 1]));
        assert!(is_safeish(vec![1, 3, 6, 7, 9]));

        assert!(is_safeish(vec![64, 66, 69, 71, 72, 72]));
        assert!(is_safeish(vec![68, 66, 69, 71, 72, 73]));
        assert!(is_safeish(vec![7, 6, 6, 3, 1]));

        assert!(is_safeish(vec![1, 2, 3, 4, 5]));
        assert!(is_safeish(vec![5, 2, 3, 4, 5]));
        assert!(is_safeish(vec![1, 5, 3, 4, 5]));
        assert!(is_safeish(vec![1, 2, 5, 4, 5]));
        assert!(is_safeish(vec![1, 2, 3, 5, 5]));
        assert!(is_safeish(vec![1, 2, 3, 4, 1]));
    }
}
