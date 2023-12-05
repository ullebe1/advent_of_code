use aoc_runner_derive::{aoc, aoc_generator};
use scan_fmt::scan_fmt;
use std::cmp::max;

#[derive(Debug)]
pub struct Input {
    games: Vec<Game>,
}

#[derive(Debug, Clone)]
pub struct Game {
    id: usize,
    rounds: Vec<Round>,
}

#[derive(Debug, Clone)]
pub struct Round {
    red: Option<usize>,
    green: Option<usize>,
    blue: Option<usize>,
}

pub enum Color {
    Red(usize),
    Green(usize),
    Blue(usize),
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Input {
    let lines = input.lines();

    let games = lines
        .map(|line| {
            let split = line.split(": ").collect::<Vec<&str>>();
            let all_rounds = split[1];
            let id = scan_fmt!(line, "Game {d}", usize).unwrap();
            println!("{}: {}", id, all_rounds);
            let rounds = all_rounds
                .split("; ")
                .map(|round| {
                    println!("{}", round);
                    let (mut red, mut green, mut blue) = (None, None, None);
                    let colors = round
                        .split(", ")
                        .map(|color| {
                            println!("{}", color);
                            let (number, color) =
                                scan_fmt!(color, "{d} {}", usize, String).unwrap();
                            match color.as_str() {
                                "red" => Color::Red(number),
                                "green" => Color::Green(number),
                                "blue" => Color::Blue(number),
                                _ => panic!("Unknown color: {}", color),
                            }
                        })
                        .collect::<Vec<Color>>();
                    for color in colors {
                        match color {
                            Color::Red(number) => red = Some(number),
                            Color::Green(number) => green = Some(number),
                            Color::Blue(number) => blue = Some(number),
                        }
                    }
                    Round { red, green, blue }
                })
                .collect::<Vec<Round>>();
            Game { id, rounds }
        })
        .collect::<Vec<Game>>();

    Input { games }
}

#[aoc(day2, part1)]
pub fn part1(input: &Input) -> usize {
    let mut sum = 0;

    for game in input.games.clone() {
        let mut impossible = false;
        for round in game.rounds {
            if round.red.unwrap_or(0) > 12
                || round.green.unwrap_or(0) > 13
                || round.blue.unwrap_or(0) > 14
            {
                impossible = true;
            }
        }

        if !impossible {
            sum += game.id;
        }
    }

    sum
}

#[aoc(day2, part2)]
pub fn part2(input: &Input) -> usize {
    let mut sum = 0;

    for game in input.games.clone() {
        let (mut red, mut green, mut blue) = (None, None, None);
        for round in game.rounds {
            if round.red.is_some() {
                if red.is_none() {
                    red = Some(round.red.unwrap());
                } else {
                    red = Some(max(red.unwrap(), round.red.unwrap()));
                }
            }
            if round.green.is_some() {
                if green.is_none() {
                    green = Some(round.green.unwrap());
                } else {
                    green = Some(max(green.unwrap(), round.green.unwrap()));
                }
            }
            if round.blue.is_some() {
                if blue.is_none() {
                    blue = Some(round.blue.unwrap());
                } else {
                    blue = Some(max(blue.unwrap(), round.blue.unwrap()));
                }
            }
        }

        let power = red.unwrap_or(1) * green.unwrap_or(1) * blue.unwrap_or(1);
        println!(
            "{}: {} * {} * {} = {}",
            game.id,
            red.unwrap(),
            green.unwrap(),
            blue.unwrap(),
            power
        );
        sum += power;
    }

    sum
}
