use crate::Aoc;
use grid::Location;
use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, Read};

#[derive(Debug)]
struct Input {
    beam_start: Location,
    splitters: HashSet<Location>,
}

fn parse(buf: &mut dyn Read) -> Input {
    let buf_reader = BufReader::new(buf);
    let mut beam_start: Option<Location> = None;
    let mut splitters = HashSet::new();
    for (y, line) in buf_reader.lines().enumerate() {
        for (x, c) in line.unwrap().chars().enumerate() {
            if c == 'S' {
                beam_start = Some(Location { x, y });
            } else if c == '^' {
                splitters.insert(Location { x, y });
            }
        }
    }

    Input {
        beam_start: beam_start.unwrap(),
        splitters,
    }
}

#[derive(Debug, Clone, Default)]
struct State {
    beams: HashMap<Location, usize>,
    hit_splitters: HashSet<Location>,
    timelines: usize,
}

fn step_beam(beam: &Location, splitters: &HashSet<Location>) -> (Option<Location>, Vec<Location>) {
    let hit_splitter = splitters
        .iter()
        .filter(|splitter| splitter.x == beam.x && splitter.y > beam.y)
        .min_by_key(|splitter| splitter.y);
    if let Some(split_location) = hit_splitter {
        let left = split_location.left();
        // Doesn't need bounds check rn as a beam off the right side of the grid won't hit
        // anything, and that's all we care about counting.
        let right = split_location.right();
        (
            Some(split_location.clone()),
            [left, Some(right)]
                .into_iter()
                .flatten()
                .filter(|beam| !splitters.contains(beam))
                .collect(),
        )
    } else {
        (None, vec![])
    }
}

fn step_beams<'a>(state: &State, splitters: &HashSet<Location>) -> State {
    let mut new_state = State {
        beams: Default::default(),
        hit_splitters: state.hit_splitters.clone(),
        timelines: state.timelines,
    };

    for (beam, strength) in &state.beams {
        let (hit_splitter, new_beams) = step_beam(&beam, splitters);
        if let Some(hit_splitter) = hit_splitter {
            new_state.hit_splitters.insert(hit_splitter);
            for beam in new_beams {
                *new_state.beams.entry(beam).or_default() += strength;
            }
        } else {
            new_state.timelines += strength;
        }
    }
    new_state
}

fn part1(buf: &mut dyn Read) {
    let Input {
        beam_start,
        splitters,
    } = parse(buf);

    let mut state = State {
        beams: HashMap::from([(beam_start, 1)]),
        ..Default::default()
    };

    while !state.beams.is_empty() {
        state = step_beams(&state, &splitters);
    }

    println!("Part 1: {}", state.hit_splitters.len());
}

fn part2(buf: &mut dyn Read) {
    let Input {
        beam_start,
        splitters,
    } = parse(buf);

    let mut state = State {
        beams: HashMap::from([(beam_start, 1)]),
        ..Default::default()
    };

    while !state.beams.is_empty() {
        state = step_beams(&state, &splitters);
    }

    println!("Part 2: {}", state.timelines);
}

inventory::submit!(Aoc::new(2025, 7, part1, part2,));
