use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
    ops::Deref,
};

use grid::Location;
use lazy_static::lazy_static;

use crate::Aoc;

fn parse(buf: &mut dyn Read) -> Vec<String> {
    let reader = BufReader::new(buf);
    reader.lines().map_while(Result::ok).collect()
}

lazy_static! {
    static ref number_pad: HashMap<char, Location> = [
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
    static ref routes: HashMap<(char, char), &'static str> = [
        (('A', 'A'), "A"),
        (('A', '<'), "v<<A"),
        (('A', '^'), "<A"),
        (('A', '>'), "vA"),
        (('A', 'v'), "<vA"),
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
        (('v', 'A'), "^>A"),
        (('v', '<'), "<A"),
        (('v', '^'), "^A"),
        (('v', '>'), ">A"),
        (('v', 'v'), "A"),
    ]
    .into_iter()
    .collect();
    static ref cheapest_routes: HashMap<(char, char), String> = {
        let mut result = HashMap::new();
        for a in number_pad.keys() {
            for b in number_pad.keys() {
                let mut route_options = vec![];

                if !(['A', '0'].contains(a) && ['1', '4', '7'].contains(b)) {
                    route_options.push(number_route(*a, *b, true, 2));
                }

                if !(['A', '0'].contains(b) && ['1', '4', '7'].contains(a)) {
                    route_options.push(number_route(*a, *b, false, 2));
                }

                route_options.sort_by_key(|route| route.len());
                result.insert((*a, *b), route_options[0].clone());
            }
        }
        result
    };
}

fn number_route(from: char, to: char, x_first: bool, steps: usize) -> String {
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

    let mut seq = seq_0;
    for _ in 0..steps {
        seq = "A"
            .chars()
            .chain(seq.chars())
            .zip(seq.chars())
            .map(|(from, to)| *routes.get(&(from, to)).unwrap())
            .collect();
    }

    seq
}

fn part1(buf: &mut dyn Read) {
    let codes = parse(buf);

    let result: usize = codes
        .iter()
        .map(|code| {
            let route: String = ['A']
                .into_iter()
                .chain(code.chars())
                .zip(code.chars())
                .map(|(from, to)| cheapest_routes.get(&(from, to)).unwrap().deref())
                .collect();
            // dbg!(code, &route);
            let result = code.strip_suffix("A").unwrap().parse::<usize>().unwrap() * route.len();
            // dbg!(result)
            result
        })
        .sum();

    println!("Part 1: {}", result);
}

lazy_static! {
    static ref transition_remap: HashMap<(char, char), Transitions> = {
        let mut remap: HashMap<(char, char), Transitions> = HashMap::new();

        for (original_transition, path) in routes.iter() {
            let new_transitions = ['A']
                .into_iter()
                .chain(path.chars())
                .zip(path.chars())
                .fold(HashMap::new(), |mut acc, transition| {
                    *acc.entry(transition).or_default() += 1;
                    acc
                });

            remap.insert(*original_transition, new_transitions);
        }

        remap
    };

    // Create a new cheapest routes list that looks further ahead
    // as 2 levels isn't deep enough to get the correct answer
    static ref cheapest_routes_2: HashMap<(char, char), String> = {
        let mut result = HashMap::new();
        for a in number_pad.keys() {
            for b in number_pad.keys() {
                let mut route_options = vec![];

                if !(['A', '0'].contains(a) && ['1', '4', '7'].contains(b)) {
                    route_options.push(number_route(*a, *b, true, 5));
                }

                if !(['A', '0'].contains(b) && ['1', '4', '7'].contains(a)) {
                    route_options.push(number_route(*a, *b, false, 5));
                }

                route_options.sort_by_key(|route| route.len());
                result.insert((*a, *b), route_options[0].clone());
            }
        }
        result
    };
}
type Transitions = HashMap<(char, char), usize>;

fn route_to_transitions(route: String) -> Transitions {
    ['A']
        .into_iter()
        .chain(route.chars())
        .zip(route.chars())
        .fold(HashMap::new(), |mut acc, transition| {
            *acc.entry(transition).or_default() += 1;
            acc
        })
}

fn new_transitions(transitions: &Transitions) -> Transitions {
    transitions
        .iter()
        .map(|(transition, count)| (transition_remap.get(transition).unwrap(), count))
        .fold(HashMap::new(), |mut acc, (transitions, multiplier)| {
            for (transition, count) in transitions {
                *acc.entry(*transition).or_default() += count * multiplier
            }
            acc
        })
}

fn part2(buf: &mut dyn Read) {
    let codes = parse(buf);

    let level_3_routes: Vec<String> = codes
        .iter()
        .map(|code| {
            let route: String = ['A']
                .into_iter()
                .chain(code.chars())
                .zip(code.chars())
                .map(|(from, to)| cheapest_routes_2.get(&(from, to)).unwrap().deref())
                .collect();
            route
        })
        .collect();

    let mut transitions: Vec<Transitions> = level_3_routes
        .into_iter()
        .map(route_to_transitions)
        .collect();

    for _ in 5..25 {
        transitions = transitions
            .into_iter()
            .map(|transitions| new_transitions(&transitions))
            .collect()
    }

    let result: usize = codes
        .iter()
        .zip(transitions.iter())
        .map(|(code, transitions)| {
            let code_val = code.strip_suffix("A").unwrap().parse::<usize>().unwrap();
            let route_length = transitions.values().sum::<usize>();
            // dbg!(code, route_length);
            code_val * route_length
        })
        .sum();

    println!("Part 2: {}", result);
}

inventory::submit!(Aoc::new(2024, 21, part1, part2,));
