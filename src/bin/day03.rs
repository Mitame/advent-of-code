use std::io::{stdin, Read};

use regex::bytes::Regex;

fn main() {
    let mut input = stdin();
    let mut buffer = vec![0u8; 4 * 1024 * 1024];
    let len = input.read(&mut buffer).unwrap();
    buffer.shrink_to(len);

    // part1(b"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
    part1(&buffer);
    part2(&buffer);
    // par
}

fn part1(buffer: &[u8]) {
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let result: u64 = regex
        .captures_iter(buffer)
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

fn part2(buffer: &[u8]) {
    let regex = Regex::new(r"do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    const DO_STR: &[u8] = b"do()";
    const DONT_STR: &[u8] = b"don't()";

    let mut do_mul = true;
    let result: u64 = regex
        .captures_iter(buffer)
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
