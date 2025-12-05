use std::io::Read;

use crate::Aoc;

type Pins = [u8; 5];

struct Data {
    locks: Vec<Pins>,
    keys: Vec<Pins>,
}

fn parse(buf: &mut dyn Read) -> Data {
    let mut buffer = Vec::new();
    buf.read_to_end(&mut buffer).unwrap();
    let buffer = String::from_utf8(buffer).unwrap();

    let mut locks = vec![];
    let mut keys = vec![];
    for pinset in buffer.split("\n\n") {
        let is_lock = pinset.starts_with("#####");
        let mut pins: Pins = [0, 0, 0, 0, 0];

        for line in pinset.lines() {
            line.chars().enumerate().for_each(|(i, p)| {
                if p == '#' {
                    pins[i] += 1;
                }
            });
        }

        for pin in pins.iter_mut() {
            *pin -= 1;
        }

        if is_lock {
            locks.push(pins);
        } else {
            keys.push(pins);
        }
    }
    Data { locks, keys }
}

fn key_fits_lock(key: &Pins, lock: &Pins) -> bool {
    key.iter().zip(lock).all(|(kp, lp)| kp + lp <= 5)
}

fn part1(buf: &mut dyn Read) {
    let Data { locks, keys } = parse(buf);
    dbg!(&locks, &keys);

    let mut results = 0;
    for lock in locks.iter() {
        for key in keys.iter() {
            if key_fits_lock(key, lock) {
                results += 1;
            }
        }
    }
    println!("Part 1: {}", results);
}

fn part2(_: &mut dyn Read) {}

inventory::submit!(Aoc::new(2024, 25, part1, part2,));
