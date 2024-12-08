use std::io::{BufRead, BufReader, Read};

use crate::Aoc;

#[derive(Debug)]
struct Balls {
    red: usize,
    green: usize,
    blue: usize,
}

#[derive(Debug)]
struct Game {
    id: usize,
    picks: Vec<Balls>,
}

fn parse_games(buf: &mut dyn Read) -> Vec<Game> {
    let reader = BufReader::new(buf);
    reader
        .lines()
        .flatten()
        .map(|line| {
            let (game_name, picks) = line.split_once(":").unwrap();
            let id = game_name.split_once(" ").unwrap().1.parse().unwrap();
            let picks = picks
                .split(";")
                .map(|pick| {
                    let balls = pick.split(",");
                    let mut red = 0;
                    let mut green = 0;
                    let mut blue = 0;

                    for ball in balls {
                        let (count, colour) = ball.trim().split_once(" ").unwrap();
                        let count: usize = count.parse().unwrap();
                        match colour {
                            "red" => red = count,
                            "green" => green = count,
                            "blue" => blue = count,
                            _ => {
                                dbg!(colour);
                            }
                        };
                    }
                    Balls { red, green, blue }
                })
                .collect();

            dbg!(line);
            dbg!(Game { id, picks })
        })
        .collect()
}

fn is_valid_game(game: &Game, max_red: usize, max_green: usize, max_blue: usize) -> bool {
    game.picks
        .iter()
        .all(|pick| pick.red <= max_red && pick.blue <= max_blue && pick.green <= max_green)
}

fn part1(buf: &mut dyn Read) {
    let games = parse_games(buf);

    let result = games
        .iter()
        .filter_map(|game| is_valid_game(game, 12, 13, 14).then_some(game.id))
        .sum::<usize>();

    println!("Part 1: {}", result);
}

fn part2(buf: &mut dyn Read) {
    let games = parse_games(buf);
    let result = games
        .into_iter()
        .map(|game| {
            game.picks
                .into_iter()
                .reduce(|acc, next| Balls {
                    red: acc.red.max(next.red),
                    green: acc.green.max(next.green),
                    blue: acc.blue.max(next.blue),
                })
                .unwrap()
        })
        .map(|min_balls| min_balls.red * min_balls.green * min_balls.blue)
        .sum::<usize>();

    println!("Part 2: {}", result)
}

inventory::submit!(Aoc::new(
    2023,
    2,
    part1,
    part2,
    include_bytes!("./inputs/day02")
));
