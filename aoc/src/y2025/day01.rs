use std::io::{BufRead, BufReader, Read};

use crate::Aoc;

type Step = i16;
type Position = i16;

fn parse(buf: &mut dyn Read) -> Vec<Step> {
    let buf_read = BufReader::new(buf);
    buf_read
        .lines()
        .map_while(|line| {
            let line = line.ok()?;
            let clicks: Step = line[1..].parse().unwrap();
            if line.starts_with("L") {
                Some(-clicks)
            } else {
                Some(clicks)
            }
        })
        .collect()
}

fn positions(steps: impl Iterator<Item = Step>) -> impl Iterator<Item = Position> {
    let mut position = 50;
    [position].into_iter().chain(steps.map(move |step| {
        position = (position + step) % 100;
        position = (position + 100) % 100;
        position
    }))
}

fn get_new_position_and_zero_passes(position: Position, step: Step) -> (Position, i16) {
    let new_position = position + step;
    let new_position_clipped = new_position.rem_euclid(100);
    if new_position < 0 {
        let zero_passes = new_position / -100;
        if position == 0 {
            (new_position_clipped, zero_passes)
        } else {
            (new_position_clipped, zero_passes + 1)
        }
    } else if new_position >= 100 {
        let zero_passes = new_position / 100;
        (new_position_clipped, zero_passes)
    } else {
        (new_position_clipped, if new_position == 0 { 1 } else { 0 })
    }
}

fn positions_with_zero_passes(
    steps: impl Iterator<Item = Step>,
) -> impl Iterator<Item = (Position, i16)> {
    let mut position = 50;
    [(position, 0)].into_iter().chain(steps.map(move |step| {
        let (new_position, zero_passes) = get_new_position_and_zero_passes(position, step);
        position = new_position;
        (new_position, zero_passes)
    }))
}

fn part1(buf: &mut dyn Read) {
    let steps = parse(buf);
    let zero_count = positions(steps.into_iter()).filter(|p| p == &0).count();
    println!("Part 1: {}", zero_count);
}

fn part2(buf: &mut dyn Read) {
    let steps = parse(buf);
    let zero_pass_count: i16 = positions_with_zero_passes(steps.into_iter())
        .map(|(_, v)| v)
        .sum();
    println!("Part 2: {}", zero_pass_count);
}

inventory::submit!(Aoc::new(2025, 1, part1, part2,));

#[cfg(test)]
mod tests {
    use crate::y2025::day01::get_new_position_and_zero_passes;

    #[test]
    fn test_get_position_and_zero_passes() {
        // Typical cases
        assert_eq!(get_new_position_and_zero_passes(50, 1000), (50, 10));
        assert_eq!(get_new_position_and_zero_passes(3, -37), (66, 1));
        assert_eq!(get_new_position_and_zero_passes(50, 150), (0, 2));
        assert_eq!(get_new_position_and_zero_passes(50, -50), (0, 1));
        assert_eq!(get_new_position_and_zero_passes(50, -150), (0, 2));

        // Start and end on zero
        assert_eq!(get_new_position_and_zero_passes(0, 100), (0, 1));
        assert_eq!(get_new_position_and_zero_passes(0, 200), (0, 2));
        assert_eq!(get_new_position_and_zero_passes(0, -100), (0, 1));
        assert_eq!(get_new_position_and_zero_passes(0, -200), (0, 2));
    }
}
