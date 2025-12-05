use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
};

use crate::Aoc;

fn parse(buf: &mut dyn Read) -> Vec<usize> {
    let mut reader = BufReader::new(buf);
    let mut line: String = String::new();
    reader.read_line(&mut line).unwrap();
    line.split(' ').map(|v| v.trim().parse().unwrap()).collect()
}

fn blink(number: usize) -> Vec<usize> {
    if number == 0 {
        vec![1]
    } else {
        let digits = number.to_string();
        if digits.len() % 2 == 0 {
            let mid = digits.len() / 2;
            vec![
                digits[..mid].parse().unwrap(),
                digits[mid..].parse().unwrap(),
            ]
        } else {
            vec![number * 2024]
        }
    }
}

fn blink_n(mut numbers: Vec<usize>, n: usize) -> Vec<usize> {
    for i in 0..n {
        eprintln!("Step {}", i);

        let new_numbers: Vec<_> = numbers.into_iter().flat_map(blink).collect();
        numbers = new_numbers;
    }

    numbers
}

fn part1(buf: &mut dyn Read) {
    let numbers = parse(buf);
    dbg!(&numbers);

    let numbers = blink_n(numbers, 25);

    println!("Part 1: {}", numbers.len());
}

type Counter<T> = HashMap<T, usize>;

fn list_to_counter<T>(items: impl Iterator<Item = T>) -> Counter<T>
where
    T: std::hash::Hash + Eq,
{
    let mut counter = HashMap::new();
    for n in items {
        *counter.entry(n).or_default() += 1;
    }
    counter
}

fn sum_counters<T>(a: Counter<T>, b: Counter<T>) -> Counter<T>
where
    T: std::hash::Hash + Eq + Clone,
{
    let mut counter = Counter::<T>::new();
    a.into_iter().chain(b).fold(&mut counter, |acc, (k, v)| {
        *acc.entry(k).or_default() += v;
        acc
    });
    counter
}

fn blink_counter_n(numbers: Vec<usize>, n: usize) -> Counter<usize> {
    let mut counter = list_to_counter(numbers.into_iter());
    for i in 0..n {
        dbg!(i);
        let new_counter = counter
            .into_iter()
            .map(|(k, multiplier)| {
                let mut counter = Counter::<usize>::new();
                blink(k)
                    .into_iter()
                    .map(|v| (v, multiplier))
                    .fold(&mut counter, |acc, (v, k)| {
                        *acc.entry(v).or_default() += k;
                        acc
                    });
                counter
            })
            .reduce(sum_counters)
            .unwrap();
        counter = new_counter;
    }

    counter
}

fn part2(buf: &mut dyn Read) {
    let numbers = parse(buf);

    let result = blink_counter_n(numbers, 75).into_values().sum::<usize>();

    println!("Part 2: {}", result);
}

inventory::submit!(Aoc::new(2024, 11, part1, part2,));

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_blink() {
        assert_eq!(blink(0), vec![1]);
        assert_eq!(blink(1), vec![2024]);
        assert_eq!(blink(10), vec![1, 0]);
        assert_eq!(blink(99), vec![9, 9]);
        assert_eq!(blink(999), vec![2021976]);
    }

    #[test]
    fn test_blink_n() {
        assert_eq!(blink_n(vec![125, 17], 1), vec![253000, 1, 7]);
        assert_eq!(blink_n(vec![125, 17], 2), vec![253, 0, 2024, 14168]);
        assert_eq!(blink_n(vec![125, 17], 3), vec![512072, 1, 20, 24, 28676032]);
        assert_eq!(
            blink_n(vec![125, 17], 4),
            vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032,]
        );
        assert_eq!(
            blink_n(vec![125, 17], 5),
            vec![1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32]
        );
        assert_eq!(
            blink_n(vec![125, 17], 6),
            vec![
                2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6,
                0, 3, 2
            ]
        );
        assert_eq!(blink_n(vec![125, 17], 25).len(), 55312);
    }

    #[test]
    fn test_blink_counter_n() {
        assert_eq!(
            blink_counter_n(vec![125, 17], 1),
            list_to_counter(vec![253000, 1, 7].into_iter())
        );
        assert_eq!(
            blink_counter_n(vec![125, 17], 2),
            list_to_counter(vec![253, 0, 2024, 14168].into_iter())
        );
        assert_eq!(
            blink_counter_n(vec![125, 17], 3),
            list_to_counter(vec![512072, 1, 20, 24, 28676032].into_iter())
        );
        assert_eq!(
            blink_counter_n(vec![125, 17], 4),
            list_to_counter(vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032,].into_iter())
        );
        assert_eq!(
            blink_counter_n(vec![125, 17], 5),
            list_to_counter(
                vec![1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32].into_iter()
            )
        );
        assert_eq!(
            blink_counter_n(vec![125, 17], 6),
            HashMap::from([
                (2097446912, 1),
                (14168, 1),
                (4048, 1),
                (2, 4),
                (0, 2),
                (4, 1),
                (40, 2),
                (48, 2),
                (2024, 1),
                (80, 1),
                (96, 1),
                (8, 1),
                (6, 2),
                (7, 1),
                (3, 1),
            ])
        );
        assert_eq!(
            blink_counter_n(vec![125, 17], 6)
                .into_values()
                .sum::<usize>(),
            22
        );
        for n in 9..25 {
            let a = blink_counter_n(vec![125, 17], n);
            let b = list_to_counter(blink_n(vec![125, 17], n).into_iter());
            for k in a.keys().chain(b.keys()) {
                if a[k] != b[k] {
                    dbg!((k, a[k], b[k]));
                }
            }
            assert_eq!(a, b);
        }
        assert_eq!(
            blink_counter_n(vec![125, 17], 25)
                .into_values()
                .sum::<usize>(),
            55312
        );
    }
}
