use aoc_runner_derive::{aoc, aoc_generator};
#[derive(Debug, Clone)]
pub enum PlayerHands {
    X,
    Y,
    Z,
}

#[derive(Debug, Clone)]
pub enum OpponentHands {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Clone)]
pub struct Input {
    rounds: Vec<(OpponentHands, PlayerHands)>,
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Input {
    let rounds = input.lines().map(|line| {
        let items: Vec<&str> = line.split_ascii_whitespace().collect();
        assert!(items.len() == 2);
        let opponent_hand = match items[0] {
            "A" => OpponentHands::Rock,
            "B" => OpponentHands::Paper,
            "C" => OpponentHands::Scissors,
            _ => unreachable!(),           
        };
        let player_hand = match items[1] {
            "X" => PlayerHands::X,
            "Y" => PlayerHands::Y,
            "Z" => PlayerHands::Z,
            _ => unreachable!()
        };
        (opponent_hand, player_hand)
    }).collect();
    Input { rounds: rounds }
}

#[aoc(day2, part1)]
pub fn part1(input: &Input) -> usize {
    input.rounds
        .clone()
        .into_iter()
        .map(|round| {
            match round {
                (OpponentHands::Rock, PlayerHands::X) => 1 + 3,
                (OpponentHands::Rock, PlayerHands::Y) => 2 + 6,
                (OpponentHands::Rock, PlayerHands::Z) => 3 + 0,
                (OpponentHands::Paper, PlayerHands::X) => 1 + 0,
                (OpponentHands::Paper, PlayerHands::Y) => 2 + 3,
                (OpponentHands::Paper, PlayerHands::Z) => 3 + 6,
                (OpponentHands::Scissors, PlayerHands::X) => 1 + 6,
                (OpponentHands::Scissors, PlayerHands::Y) => 2 + 0,
                (OpponentHands::Scissors, PlayerHands::Z) => 3 + 3,
            }
        })
        .sum()
}

#[aoc(day2, part2)]
pub fn part2(input: &Input) -> usize {
    input.rounds
        .clone()
        .into_iter()
        .map(|round| {
            match round {
                (OpponentHands::Rock, PlayerHands::X) => 0 + 3,
                (OpponentHands::Rock, PlayerHands::Y) => 3 + 1,
                (OpponentHands::Rock, PlayerHands::Z) => 6 + 2,
                (OpponentHands::Paper, PlayerHands::X) => 0 + 1,
                (OpponentHands::Paper, PlayerHands::Y) => 3 + 2,
                (OpponentHands::Paper, PlayerHands::Z) => 6 + 3,
                (OpponentHands::Scissors, PlayerHands::X) => 0 + 2,
                (OpponentHands::Scissors, PlayerHands::Y) => 3 + 3,
                (OpponentHands::Scissors, PlayerHands::Z) => 6 + 1,
            }
        })
        .sum()
}
