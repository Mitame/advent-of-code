use std::io::{BufRead, BufReader, Read};

use grid::Location;

use crate::Aoc;

fn parse(buf: &mut dyn Read) -> Vec<Location> {
    let buf_reader = BufReader::new(buf);
    buf_reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut cells = line.split(",").map(|v| v.parse::<usize>().unwrap());
            Location {
                x: cells.next().unwrap(),
                y: cells.next().unwrap(),
            }
        })
        .collect()
}

fn area(a: &Location, b: &Location) -> usize {
    let diagonal = a.abs_diff(b);
    let side_lengths = Location {
        x: diagonal.x + 1,
        y: diagonal.y + 1,
    };

    side_lengths.x * side_lengths.y
}

fn part1(buf: &mut dyn Read) {
    let points = parse(buf);
    let mut biggest_rect = 0;
    for (i, a) in points.iter().enumerate() {
        for b in points[(i + 1)..].iter() {
            let rect_size = area(a, b);
            biggest_rect = biggest_rect.max(rect_size)
        }
    }

    println!("Part 1: {}", biggest_rect)
}

fn is_within_rect(point: &Location, rect_a: &Location, rect_b: &Location) -> bool {
    return (rect_a.x < rect_b.x && rect_a.x < point.x && point.x < rect_b.x
        || rect_a.x > rect_b.x && rect_a.x > point.x && point.x > rect_b.x)
        && (rect_a.y < rect_b.y && rect_a.y < point.y && point.y < rect_b.y
            || rect_a.y > rect_b.y && rect_a.y > point.y && point.y > rect_b.y);
}

fn part2(buf: &mut dyn Read) {
    let points = parse(buf);

    // To determine if a rectangle leaks outside the bounds drawn by the points,
    // we calculate all points along the border of the shape, then check if any
    // point from that border is inside the given rectangle (not along one of its edges).
    // This works so long as two borders of the shape are not touching, as the tile pattern
    // in that case would be permissable (but thankfully the designers of this puzzle aren't
    // that much of a bastard)

    // Calculate the points along the border of the shape (out of order)
    let mut border_points = vec![];
    for (a, b) in points
        .iter()
        .zip(points[1..].iter())
        .chain([(points.last().unwrap(), points.first().unwrap())])
    {
        let (start, end) = match a.x > b.x || a.y > b.y {
            true => (b, a),
            false => (a, b),
        };
        let diff = end - start;
        if diff.x > 0 {
            for i in 0..diff.x {
                border_points.push(Location {
                    x: start.x + i as usize,
                    y: start.y,
                })
            }
        } else {
            for i in 0..diff.y {
                border_points.push(Location {
                    x: start.x,
                    y: start.y + i as usize,
                })
            }
        }
    }

    // Find the biggest rectangle as before
    let mut biggest_rect = 0;
    for (i, a) in points.iter().enumerate() {
        dbg!(i);
        for b in points[(i + 1)..].iter() {
            if border_points
                .iter()
                .any(|point| is_within_rect(point, a, b))
            {
                continue;
            }
            let rect_size = area(a, b);
            biggest_rect = biggest_rect.max(rect_size)
        }
    }

    println!("Part 2: {}", biggest_rect)
}

inventory::submit!(Aoc::new(2025, 9, part1, part2,));
