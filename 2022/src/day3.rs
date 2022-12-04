use std::collections::HashSet;
use itertools::Itertools;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone)]
pub struct Input {
    rounds: Vec<Vec<char>>,
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Input {
    let rounds = input
        .lines()
        .map(|line| {
            line.chars().into_iter().collect::<Vec<char>>()
        }).collect();
    Input { rounds: rounds }
}

#[aoc(day3, part1)]
pub fn part1(input: &Input) -> usize {
    input
        .rounds
        .clone()
        .into_iter()
        .map(|line| {
            let mut chars: Vec<char> = line.into_iter().collect();
            let second_half = chars.split_off(chars.len() / 2).into_iter().collect::<HashSet<char>>();
            let first_half = chars.into_iter().collect::<HashSet<char>>();
            let intersection = first_half.intersection(&second_half).collect::<Vec<&char>>();
            assert_eq!(intersection.len(), 1);
            intersection[0].clone()
        })
        .map(|item| {
            if item.is_ascii_lowercase() {
                (item as usize) - 96
            } else {
                (item as usize) - 38
            }
        }).sum()
}

#[aoc(day3, part2)]
pub fn part2(input: &Input) -> usize {
    input
        .rounds
        .clone()
        .into_iter()
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            let rucksacks = chunk.collect::<Vec<_>>();
            assert_eq!(rucksacks.len(), 3);

            // Define rucksacks to help lifetimes
            let rucksack_1 = rucksacks[0].to_owned().into_iter().collect::<HashSet<_>>();
            let rucksack_2 = rucksacks[1].to_owned().into_iter().collect::<HashSet<_>>();
            let rucksack_3 = (&rucksacks[2]).into_iter().collect::<HashSet<_>>();

            // Intersections to find common item to all three
            let intersection_1 = rucksack_1.intersection(&rucksack_2).collect::<HashSet<_>>();
            let intersection_2 = intersection_1.intersection(&rucksack_3).collect::<Vec<_>>();
            assert_eq!(intersection_2.len(), 1);

            // The badge
            **intersection_2[0]
        })
        .map(|badge| {
            if badge.is_ascii_lowercase() {
                (badge as usize) - 96
            } else {
                (badge as usize) - 38
            }
        })
        .sum()
}
