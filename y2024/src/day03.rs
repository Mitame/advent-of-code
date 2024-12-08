use crate::Aoc;
use std::io::Read;

use regex::bytes::Regex;

fn parse(buf: &mut dyn Read) -> Vec<u8> {
    let mut buffer = vec![0u8; 4 * 1024 * 1024];
    let len = buf.read(&mut buffer).unwrap();
    buffer.shrink_to(len);

    buffer
}

fn part1(buf: &mut dyn Read) {
    let buffer = parse(buf);

    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let result: u64 = regex
        .captures_iter(&buffer)
        .map(|c| {
            let a: u64 = String::from_utf8(c.get(1).unwrap().as_bytes().to_vec())
                .unwrap()
                .parse()
                .unwrap();
            let b: u64 = String::from_utf8(c.get(2).unwrap().as_bytes().to_vec())
                .unwrap()
                .parse()
                .unwrap();

            a * b
        })
        .sum();

    println!("Part 1: {}", result);
}
inventory::submit!(Aoc::new(3, 1, part1));

fn part2(buf: &mut dyn Read) {
    let buffer = parse(buf);

    let regex = Regex::new(r"do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    const DO_STR: &[u8] = b"do()";
    const DONT_STR: &[u8] = b"don't()";

    let mut do_mul = true;
    let result: u64 = regex
        .captures_iter(&buffer)
        .map(|c| {
            if c.get(0).unwrap().as_bytes() == DO_STR {
                do_mul = true;
                return 0;
            } else if c.get(0).unwrap().as_bytes() == DONT_STR {
                do_mul = false;
                return 0;
            }

            if !do_mul {
                return 0;
            }

            let a: u64 = String::from_utf8(c.get(1).unwrap().as_bytes().to_vec())
                .unwrap()
                .parse()
                .unwrap();
            let b: u64 = String::from_utf8(c.get(2).unwrap().as_bytes().to_vec())
                .unwrap()
                .parse()
                .unwrap();

            a * b
        })
        .sum();

    println!("Part 2: {}", result);
}
inventory::submit!(Aoc::new(3, 2, part2));
