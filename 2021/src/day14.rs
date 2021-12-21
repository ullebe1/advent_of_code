use std::{collections::HashMap};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

pub struct Input {
    template: Vec<char>,
    rules: HashMap<(char, char), char>,
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Input {
    let mut lines = input.lines();
    let template = lines.next().unwrap().chars().collect();

    lines.advance_by(1).unwrap();

    let mut rules = HashMap::new();

    for line in lines {
        let mut parts = line.split_ascii_whitespace();
        let mut pair = parts.next().unwrap().chars();
        parts.advance_by(1).unwrap();
        let mut insert = parts.next().unwrap().chars();
        rules
            .try_insert(
                (pair.next().unwrap(), pair.next().unwrap()),
                insert.next().unwrap(),
            )
            .unwrap();
    }

    Input {
        template: template,
        rules: rules,
    }
}

#[aoc(day14, part1, naive)]
pub fn part1_naive(input: &Input) -> usize {
    let template = input.template.clone();
    let rules = input.rules.clone();

    solve_naive(template, rules, 10)
}

#[aoc(day14, part1, clever)]
pub fn part1_clever(input: &Input) -> usize {
    let template = input.template.clone();
    let rules = input.rules.clone();

    solve_clever(template, rules, 10)
}

// Too slow, disabled
// #[aoc(day14, part2, naive)]
// pub fn part2_naive(input: &Input) -> usize {
//     let template = input.template.clone();
//     let rules = input.rules.clone();

//     solve_naive(template, rules, 40)
// }

#[aoc(day14, part2, clever)]
pub fn part2_clever(input: &Input) -> usize {
    let template = input.template.clone();
    let rules = input.rules.clone();

    solve_clever(template, rules, 40)
}

fn solve_naive(mut template: Vec<char>, rules: HashMap<(char, char), char>, iterations: usize) -> usize {
    for i in 0..iterations {
        println!("Starting iteration {}", i);
        template = {
            let inserts = template
                .windows(2)
                .map(|x| *rules.get(&(x[0], x[1])).unwrap())
                .collect::<Vec<char>>();
            template.into_iter().interleave(inserts.into_iter()).collect()
        }
    }
    
    let mut occurences = HashMap::new();
    for char in template {
        *occurences.entry(char).or_insert(0) += 1;
    }

    match occurences.into_iter().minmax_by_key(|&(_, count)| count) {
        itertools::MinMaxResult::MinMax(min, max) => max.1 - min.1,
        _ => unreachable!()
    }    
}

fn solve_clever(template: Vec<char>, rules: HashMap<(char, char), char>, iterations: usize) -> usize {
    let mut pairs: HashMap<(char, char), usize> = HashMap::new();
    for pair in template.windows(2) {
        *pairs.entry((pair[0], pair[1])).or_insert(0) += 1;
    }

    for i in 0..iterations {
        println!("Starting iteration {}", i);
        let mut new_pairs = HashMap::new();
        for ((a, b), value) in pairs {
            let c = *rules.get(&(a, b)).unwrap();
            *new_pairs.entry((a, c)).or_insert(0) += value;
            *new_pairs.entry((c, b)).or_insert(0) += value;
        }
        pairs = new_pairs
    }
    
    let mut occurences = HashMap::new();
    for (pair, value) in pairs {
        *occurences.entry(pair.0).or_insert(0) += value;
        *occurences.entry(pair.1).or_insert(0) += value;
    }

    match occurences.into_iter().minmax_by_key(|&(_, count)| count) {
        itertools::MinMaxResult::MinMax(min, max) => {println!("{}, {}", min.1, max.1); ((max.1 - min.1) as f64 / 2 as f64).round() as usize},
        _ => unreachable!()
    }    
}