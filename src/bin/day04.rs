use std::io::{stdin, BufRead, BufReader};

fn main() {
    let reader = BufReader::new(stdin());
    let mut lines = Vec::new();
    let mut line_length = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        line_length = line.len();
        lines.push(line);
    }
    let body = lines.join("");
    dbg!(&body.len());

    part1(&body, line_length);
    part2(&body, line_length);
}

#[derive(Clone, Debug)]
struct Coord {
    x: usize,
    y: usize,
}

fn index_to_coord(line_length: usize, index: usize) -> Coord {
    return Coord {
        x: index % line_length,
        y: index / line_length,
    };
}

fn coord_to_index(line_length: usize, coord: &Coord) -> usize {
    return coord.y * line_length + coord.x;
}

fn move_left(coord: &Coord) -> Option<Coord> {
    if coord.x > 0 {
        Some(Coord {
            x: coord.x - 1,
            y: coord.y,
        })
    } else {
        None
    }
}

fn move_right(line_length: usize, coord: &Coord) -> Option<Coord> {
    if coord.x + 1 < line_length {
        Some(Coord {
            x: coord.x + 1,
            y: coord.y,
        })
    } else {
        None
    }
}

fn move_down(coord: &Coord) -> Coord {
    Coord {
        x: coord.x,
        y: coord.y + 1,
    }
}

fn move_up(coord: &Coord) -> Option<Coord> {
    if coord.y > 0 {
        Some(Coord {
            x: coord.x,
            y: coord.y - 1,
        })
    } else {
        None
    }
}

fn check_down(body: &str, line_length: usize, search_text: &str, start: &Coord) -> bool {
    let mut locations = vec![start.clone()];
    for _ in 1..search_text.len() {
        locations.push(move_down(locations.last().unwrap()))
    }

    let target_str: Option<String> = locations
        .iter()
        .map(|coord| body.chars().nth(coord_to_index(line_length, coord)))
        .collect();
    match target_str {
        None => false,
        Some(target) => &target == search_text,
    }
}

fn check_right(body: &str, line_length: usize, search_text: &str, start: &Coord) -> bool {
    let mut locations = vec![start.clone()];
    for _ in 1..search_text.len() {
        let new_location = move_right(line_length, locations.last().unwrap());
        if let Some(new_location) = new_location {
            locations.push(new_location);
        } else {
            return false;
        }
    }

    let target_str: Option<String> = locations
        .iter()
        .map(|coord| body.chars().nth(coord_to_index(line_length, coord)))
        .collect();
    match target_str {
        None => false,
        Some(target) => &target == search_text,
    }
}

fn check_diagonal_left(body: &str, line_length: usize, search_text: &str, start: &Coord) -> bool {
    let mut locations = vec![start.clone()];
    for _ in 1..search_text.len() {
        let new_location = move_left(&move_down(locations.last().unwrap()));
        if let Some(new_location) = new_location {
            locations.push(new_location);
        } else {
            return false;
        }
    }

    let target_str: Option<String> = locations
        .iter()
        .map(|coord| body.chars().nth(coord_to_index(line_length, coord)))
        .collect();
    match target_str {
        None => false,
        Some(target) => &target == search_text,
    }
}

fn check_diagonal_right(body: &str, line_length: usize, search_text: &str, start: &Coord) -> bool {
    let mut locations = vec![start.clone()];
    for _ in 1..search_text.len() {
        let new_location = move_right(line_length, &move_down(locations.last().unwrap()));
        if let Some(new_location) = new_location {
            locations.push(new_location);
        } else {
            return false;
        }
    }

    let target_str: Option<String> = locations
        .iter()
        .map(|coord| body.chars().nth(coord_to_index(line_length, coord)))
        .collect();
    match target_str {
        None => false,
        Some(target) => &target == search_text,
    }
}

fn check_diagonal_up_right(
    body: &str,
    line_length: usize,
    search_text: &str,
    start: &Coord,
) -> bool {
    let mut locations = vec![start.clone()];
    for _ in 1..search_text.len() {
        let new_location =
            move_up(locations.last().unwrap()).and_then(|coord| move_right(line_length, &coord));
        if let Some(new_location) = new_location {
            locations.push(new_location);
        } else {
            return false;
        }
    }

    let target_str: Option<String> = locations
        .iter()
        .map(|coord| body.chars().nth(coord_to_index(line_length, coord)))
        .collect();
    match target_str {
        None => false,
        Some(target) => &target == search_text,
    }
}

fn check_diagonal_up_left(
    body: &str,
    line_length: usize,
    search_text: &str,
    start: &Coord,
) -> bool {
    let mut locations = vec![start.clone()];
    for _ in 1..search_text.len() {
        let new_location = move_up(locations.last().unwrap()).and_then(|coord| move_left(&coord));
        if let Some(new_location) = new_location {
            locations.push(new_location);
        } else {
            return false;
        }
    }

    let target_str: Option<String> = locations
        .iter()
        .map(|coord| body.chars().nth(coord_to_index(line_length, coord)))
        .collect();
    match target_str {
        None => false,
        Some(target) => &target == search_text,
    }
}

fn part1(body: &str, line_length: usize) {
    let mut hit_count = 0;
    for (i, _) in body.char_indices() {
        let coord = index_to_coord(line_length, i);
        let hits = [
            check_down(&body, line_length, "XMAS", &coord),
            check_right(&body, line_length, "XMAS", &coord),
            check_diagonal_left(&body, line_length, "XMAS", &coord),
            check_diagonal_right(&body, line_length, "XMAS", &coord),
            check_down(&body, line_length, "SAMX", &coord),
            check_right(&body, line_length, "SAMX", &coord),
            check_diagonal_left(&body, line_length, "SAMX", &coord),
            check_diagonal_right(&body, line_length, "SAMX", &coord),
        ];

        let hits = hits.into_iter().filter(|hit| *hit).count();
        hit_count += hits
    }

    println!("Part 1: {}", hit_count);
}

fn part2(body: &str, line_length: usize) {
    let mut hit_count = 0;
    for (i, _) in body.char_indices() {
        let start = index_to_coord(line_length, i);
        let hit = check_diagonal_up_left(&body, line_length, "AS", &start)
            && check_diagonal_up_right(&body, line_length, "AS", &start)
            && check_diagonal_left(&body, line_length, "AM", &start)
            && check_diagonal_right(&body, line_length, "AM", &start)
            || check_diagonal_up_left(&body, line_length, "AM", &start)
                && check_diagonal_up_right(&body, line_length, "AS", &start)
                && check_diagonal_left(&body, line_length, "AM", &start)
                && check_diagonal_right(&body, line_length, "AS", &start)
            || check_diagonal_up_left(&body, line_length, "AM", &start)
                && check_diagonal_up_right(&body, line_length, "AM", &start)
                && check_diagonal_left(&body, line_length, "AS", &start)
                && check_diagonal_right(&body, line_length, "AS", &start)
            || check_diagonal_up_left(&body, line_length, "AS", &start)
                && check_diagonal_up_right(&body, line_length, "AM", &start)
                && check_diagonal_left(&body, line_length, "AS", &start)
                && check_diagonal_right(&body, line_length, "AM", &start);

        if hit {
            hit_count += 1;
        }
    }

    println!("Part 1: {}", hit_count);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_check_right() {
        let body = "AAAAXMASAAAA";
        assert!(check_right(body, 4, "XMAS", &Coord { x: 0, y: 1 }))
    }

    #[test]
    fn test_check_down() {
        let body = "AXAAMAAAAASA";
        assert!(check_down(body, 3, "XMAS", &Coord { x: 1, y: 0 }))
    }

    #[test]
    fn test_check_diagonal_left() {
        let body = "ABCXDEMFGAHISJKL";
        assert!(check_diagonal_left(body, 4, "XMAS", &Coord { x: 3, y: 0 }))
    }

    #[test]
    fn test_check_diagonal_right() {
        let body = "XBCDEMGHIJALMNOS";
        assert!(check_diagonal_right(body, 4, "XMAS", &Coord { x: 0, y: 0 }))
    }
}
