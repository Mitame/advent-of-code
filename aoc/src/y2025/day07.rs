use std::io::{Read, BufReader, BufRead};
use std::collections::HashSet;
use crate::Aoc;
use grid::Location;

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
                beam_start = Some(Location {
                    x,
                    y
                });
            } else if c == '^' {
                splitters.insert(Location {
                    x, y
                });
            }
        } 
    }

    Input {
        beam_start: beam_start.unwrap(),
        splitters,
    }
}

fn step_beam(beam: &Location, splitters: &HashSet<Location>) -> (Option<Location>, Vec<Location>) {
    let hit_splitter = splitters.iter().filter(|splitter| splitter.x == beam.x && splitter.y > beam.y).min_by_key(|splitter| splitter.y);
    if let Some(split_location) = hit_splitter {
        let left = split_location.left();
        // Doesn't need bounds check rn as a beam off the right side of the grid won't hit
        // anything, and that's all we care about counting.
        let right = split_location.right();
        (Some(split_location.clone()), [left, Some(right)].into_iter().flatten().filter(|beam| !splitters.contains(beam)).collect())
    } else {
        (None, vec![])
    }
}

fn step_beams<'a>(beams: impl Iterator<Item=&'a Location>, splitters: &HashSet<Location>) -> (HashSet<Location>, HashSet<Location>) {
    beams.map(|beam| step_beam(beam, splitters)).fold((HashSet::new(), HashSet::new()), |(mut all_hit_splitters, mut all_beams), (hit_splitter, beams)| {
        all_hit_splitters.extend(hit_splitter.into_iter());
        all_beams.extend(beams);
        (all_hit_splitters, all_beams)
    })
}

fn part1(buf: &mut dyn Read) {
    let Input { beam_start, splitters } = parse(buf);
    let mut beams = HashSet::from([beam_start]);
    let mut all_hit_splitters = HashSet::new();
    loop {
        let (new_hits, new_beams) = step_beams(beams.iter(), &splitters);
        if new_hits.is_empty() {
            break;
        }
        all_hit_splitters.extend(new_hits);
        beams = new_beams;
    }
    
    println!("Part 1: {}", all_hit_splitters.len());
}

fn part2(buf: &mut dyn Read) {

}

inventory::submit!(Aoc::new(
    2025,
    7,
    part1,
    part2,
));

