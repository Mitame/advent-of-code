use std::io::{BufRead, BufReader, Read};

use grid::{Grid, Location, Offset};

use crate::Aoc;

#[derive(Debug, PartialEq)]
enum Content {
    Roll,
    Nothing,
}

fn parse(buf: &mut dyn Read) -> Grid<Content> {
    let reader = BufReader::new(buf);
    let mut lines = Vec::new();
    let mut line_length = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        line_length = line.len();
        lines.push(line);
    }
    Grid::new(
        lines.join("").chars().map(|char| match char {
            '@' => Content::Roll,
            _ => Content::Nothing,
        }),
        line_length,
    )
}

fn adjacent(location: &Location) -> impl Iterator<Item = Location> + use<'_> {
    [
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ]
    .into_iter()
    .filter_map(move |(x, y)| {
        if -x > location.x as isize {
            None
        } else if -y > location.y as isize {
            None
        } else {
            Some(location + &Offset { x, y })
        }
    })
}

fn is_movable_roll(grid: &Grid<Content>, location: &Location) -> bool {
    let content = grid.get(location);
    if content != Some(&Content::Roll) {
        return false;
    }

    let adjacent_roll_count = adjacent(location)
        .filter(|location| {
            let content = grid.get(&location);
            content == Some(&Content::Roll)
        })
        .count();
    adjacent_roll_count < 4
}

fn part1(buf: &mut dyn Read) {
    let grid = parse(buf);
    let movable_roll_count = grid
        .iter_locations()
        .filter(|location| is_movable_roll(&grid, location))
        .count();
    println!("Part 1: {movable_roll_count}");
}

fn remove_rolls(grid: &Grid<Content>) -> (Grid<Content>, usize) {
    let mut removed_count = 0;
    let new_grid = Grid::new(
        grid.iter_locations().map(|location| {
            let content = grid.get(&location).unwrap();
            if content == &Content::Nothing {
                Content::Nothing
            } else if is_movable_roll(grid, &location) {
                removed_count += 1;
                return Content::Nothing;
            } else {
                return Content::Roll;
            }
        }),
        grid.width(),
    );
    return (new_grid, removed_count);
}

fn part2(buf: &mut dyn Read) {
    let mut grid = parse(buf);
    let mut removed_rolls = 0;
    loop {
        let (new_grid, removed_count) = remove_rolls(&grid);
        if removed_count == 0 {
            break;
        }
        removed_rolls += removed_count;
        grid = new_grid
    }
    println!("Part 2: {removed_rolls}")
}

inventory::submit!(Aoc::new(2025, 4, part1, part2,));

#[cfg(test)]
mod tests {
    use crate::y2025::day04::{is_movable_roll, parse};

    #[test]
    fn example() {
        let data = b"..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";
        let grid = parse(&mut &data[..]);
        let movable_roll_count = grid
            .iter_locations()
            .filter(|location| is_movable_roll(&grid, location))
            .count();
        assert_eq!(movable_roll_count, 13);
    }
}
