use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[derive(Debug)]
pub struct Input {
    cards: Vec<Card>,
}

#[derive(Debug, Clone)]
pub struct Card {
    winning_numbers: HashSet<usize>,
    drawn_numbers: HashSet<usize>,
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Input {
    let lines = input.lines();

    let cards = lines
        .map(|line| {
            let (_, numbers) = line.split_once(": ").unwrap();
            let (winning_numbers, drawn_numbers) = numbers.split_once(" | ").unwrap();
            let winning_numbers = winning_numbers
                .split(" ")
                .filter(|n| !n.is_empty())
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<HashSet<usize>>();
            let drawn_numbers = drawn_numbers
                .split(" ")
                .filter(|n| !n.is_empty())
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<HashSet<usize>>();
            Card {
                winning_numbers,
                drawn_numbers,
            }
        })
        .collect::<Vec<Card>>();

    Input { cards }
}

#[aoc(day4, part1)]
pub fn part1(input: &Input) -> usize {
    let mut points = 0;

    for card in input.cards.clone() {
        let matches = card
            .winning_numbers
            .intersection(&card.drawn_numbers)
            .collect::<HashSet<&usize>>()
            .len();
        points += 2_isize.pow(matches as u32 - 1);
    }

    points as usize
}

#[aoc(day4, part2)]
pub fn part2(input: &Input) -> usize {
    let mut points = 0;

    let mut multipliers = vec![1; input.cards.len()];

    for card in input.cards.clone().iter().enumerate() {
        let matches = card
            .1
            .winning_numbers
            .intersection(&card.1.drawn_numbers)
            .collect::<HashSet<&usize>>()
            .len();
        let range = card.0 + 1..matches + card.0 + 1;
        for i in range {
            multipliers[i] += multipliers[card.0];
        }
    }
    points += multipliers.iter().sum::<usize>();

    points as usize
}
