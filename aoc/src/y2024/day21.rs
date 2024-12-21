use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
    ops::Deref,
};

use grid::Location;

use crate::Aoc;

fn parse(buf: &mut dyn Read) -> Vec<String> {
    let reader = BufReader::new(buf);
    reader.lines().map_while(Result::ok).collect()
}
fn part1(buf: &mut dyn Read) {
    let number_pad: HashMap<char, Location> = [
        ('A', Location { x: 2, y: 3 }),
        ('0', Location { x: 1, y: 3 }),
        ('1', Location { x: 0, y: 2 }),
        ('2', Location { x: 1, y: 2 }),
        ('3', Location { x: 2, y: 2 }),
        ('4', Location { x: 0, y: 1 }),
        ('5', Location { x: 1, y: 1 }),
        ('6', Location { x: 2, y: 1 }),
        ('7', Location { x: 0, y: 0 }),
        ('8', Location { x: 1, y: 0 }),
        ('9', Location { x: 2, y: 0 }),
    ]
    .into_iter()
    .collect();

    let routes: HashMap<(char, char), &str> = [
        (('A', 'A'), "A"),
        (('A', '<'), "v<<A"),
        (('A', '^'), "<A"),
        (('A', '>'), "vA"),
        (('A', 'v'), "v<A"),
        (('<', 'A'), ">>^A"),
        (('<', '<'), "A"),
        (('<', '^'), ">^A"),
        (('<', '>'), ">>A"),
        (('<', 'v'), ">A"),
        (('^', 'A'), ">A"),
        (('^', '<'), "v<A"),
        (('^', '^'), "A"),
        (('^', '>'), "v>A"),
        (('^', 'v'), "vA"),
        (('>', 'A'), "^A"),
        (('>', '<'), "<<A"),
        (('>', '^'), "<^A"),
        (('>', '>'), "A"),
        (('>', 'v'), "<A"),
        (('v', 'A'), ">^A"),
        (('v', '<'), "<A"),
        (('v', '^'), "^A"),
        (('v', '>'), ">A"),
        (('v', 'v'), "A"),
    ]
    .into_iter()
    .collect();

    let codes = parse(buf);

    let number_route = |from: char, to: char, x_first: bool| {
        let from_location = number_pad.get(&from).unwrap();
        let to_location = number_pad.get(&to).unwrap();
        let offset = to_location - from_location;
        let x_char = if offset.x < 0 { '<' } else { '>' };
        let y_char = if offset.y < 0 { '^' } else { 'v' };

        let seq_0: String = if x_first {
            [x_char]
                .into_iter()
                .cycle()
                .take(offset.x.unsigned_abs())
                .chain([y_char].into_iter().cycle().take(offset.y.unsigned_abs()))
                .chain(['A'])
                .collect()
        } else {
            [y_char]
                .into_iter()
                .cycle()
                .take(offset.y.unsigned_abs())
                .chain([x_char].into_iter().cycle().take(offset.x.unsigned_abs()))
                .chain(['A'])
                .collect()
        };

        let seq_1: String = "A"
            .chars()
            .chain(seq_0.chars())
            .zip(seq_0.chars())
            .map(|(from, to)| *routes.get(&(from, to)).unwrap())
            .collect();
        let seq_2: String = "A"
            .chars()
            .chain(seq_1.chars())
            .zip(seq_1.chars())
            .map(|(from, to)| *routes.get(&(from, to)).unwrap())
            .collect();
        seq_2
    };

    let mut cheapest_routes = HashMap::new();
    for a in number_pad.keys() {
        for b in number_pad.keys() {
            let mut routes = vec![];

            if !(['A', '0'].contains(a) && ['1', '4', '7'].contains(b)) {
                routes.push(number_route(*a, *b, true));
            }

            if !(['A', '0'].contains(b) && ['1', '4', '7'].contains(a)) {
                routes.push(number_route(*a, *b, false));
            }

            routes.sort_by_key(|route| route.len());
            cheapest_routes.insert((*a, *b), routes[0].clone());
        }
    }

    let result: usize = codes
        .iter()
        .map(|code| {
            let route: String = ['A']
                .into_iter()
                .chain(code.chars())
                .zip(code.chars())
                .map(|(from, to)| cheapest_routes.get(&(from, to)).unwrap().deref())
                .collect();
            dbg!(code, &route);
            let result = code.strip_suffix("A").unwrap().parse::<usize>().unwrap() * route.len();
            dbg!(result)
        })
        .sum();

    println!("Part 1: {}", result);
}

fn part2(buf: &mut dyn Read) {
    let number_pad: HashMap<char, Location> = [
        ('A', Location { x: 2, y: 3 }),
        ('0', Location { x: 1, y: 3 }),
        ('1', Location { x: 0, y: 2 }),
        ('2', Location { x: 1, y: 2 }),
        ('3', Location { x: 2, y: 2 }),
        ('4', Location { x: 0, y: 1 }),
        ('5', Location { x: 1, y: 1 }),
        ('6', Location { x: 2, y: 1 }),
        ('7', Location { x: 0, y: 0 }),
        ('8', Location { x: 1, y: 0 }),
        ('9', Location { x: 2, y: 0 }),
    ]
    .into_iter()
    .collect();

    let routes: HashMap<(char, char), &str> = [
        (('A', 'A'), "A"),
        (('A', '<'), "v<<A"),
        (('A', '^'), "<A"),
        (('A', '>'), "vA"),
        (('A', 'v'), "v<A"),
        (('<', 'A'), ">>^A"),
        (('<', '<'), "A"),
        (('<', '^'), ">^A"),
        (('<', '>'), ">>A"),
        (('<', 'v'), ">A"),
        (('^', 'A'), ">A"),
        (('^', '<'), "v<A"),
        (('^', '^'), "A"),
        (('^', '>'), "v>A"),
        (('^', 'v'), "vA"),
        (('>', 'A'), "^A"),
        (('>', '<'), "<<A"),
        (('>', '^'), "<^A"),
        (('>', '>'), "A"),
        (('>', 'v'), "<A"),
        (('v', 'A'), ">^A"),
        (('v', '<'), "<A"),
        (('v', '^'), "^A"),
        (('v', '>'), ">A"),
        (('v', 'v'), "A"),
    ]
    .into_iter()
    .collect();

    let codes = parse(buf);

    let number_route = |from: char, to: char, x_first: bool| {
        let from_location = number_pad.get(&from).unwrap();
        let to_location = number_pad.get(&to).unwrap();
        let offset = to_location - from_location;
        let x_char = if offset.x < 0 { '<' } else { '>' };
        let y_char = if offset.y < 0 { '^' } else { 'v' };

        let seq_0: String = if x_first {
            [x_char]
                .into_iter()
                .cycle()
                .take(offset.x.unsigned_abs())
                .chain([y_char].into_iter().cycle().take(offset.y.unsigned_abs()))
                .chain(['A'])
                .collect()
        } else {
            [y_char]
                .into_iter()
                .cycle()
                .take(offset.y.unsigned_abs())
                .chain([x_char].into_iter().cycle().take(offset.x.unsigned_abs()))
                .chain(['A'])
                .collect()
        };

        let mut it_gets_worse = seq_0;

        for _ in 0..25 {
            it_gets_worse = "A"
                .chars()
                .chain(it_gets_worse.chars())
                .zip(it_gets_worse.chars())
                .map(|(from, to)| *routes.get(&(from, to)).unwrap())
                .collect();
        }
        it_gets_worse
    };

    let mut cheapest_routes = HashMap::new();
    for a in number_pad.keys() {
        for b in number_pad.keys() {
            let mut routes = vec![];

            if !(['A', '0'].contains(a) && ['1', '4', '7'].contains(b)) {
                routes.push(number_route(*a, *b, true));
            }

            if !(['A', '0'].contains(b) && ['1', '4', '7'].contains(a)) {
                routes.push(number_route(*a, *b, false));
            }

            routes.sort_by_key(|route| route.len());
            cheapest_routes.insert((*a, *b), routes[0].clone());
            eprintln!("Completed {} to {}", a, b);
        }
    }

    let result: usize = codes
        .iter()
        .map(|code| {
            let route: String = ['A']
                .into_iter()
                .chain(code.chars())
                .zip(code.chars())
                .map(|(from, to)| cheapest_routes.get(&(from, to)).unwrap().deref())
                .collect();
            dbg!(code, &route);
            let result = code.strip_suffix("A").unwrap().parse::<usize>().unwrap() * route.len();
            dbg!(result)
        })
        .sum();

    println!("Part 2: {}", result);
}

inventory::submit!(Aoc::new(
    2024,
    21,
    part1,
    part2,
    include_bytes!("./inputs/day21")
));
