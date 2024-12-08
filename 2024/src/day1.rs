use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
pub struct Input {
    left: Vec<usize>,
    right: Vec<usize>,
}

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Input {
    let lines = input.lines();

    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in lines {
        let mut parts = line.split_whitespace();
        let left_part = parts.next().unwrap().parse::<usize>().unwrap();
        let right_part = parts.next().unwrap().parse::<usize>().unwrap();

        left.push(left_part);
        right.push(right_part);
    }

    Input { left, right }
}

#[aoc(day1, part1)]
pub fn part1(input: &Input) -> usize {
    let mut left = input.left.clone();
    let mut right = input.right.clone();

    left.sort();
    right.sort();

    let pairs = left.iter().zip(right.iter());

    println!("{:?}", pairs);

    let sum: usize = pairs.map(
        |(l, r)| {
            l.abs_diff(*r)
        }
    ).sum();


    sum
}

#[aoc(day1, part2)]
pub fn part2(input: &Input) -> usize {
    let left = input.left.clone();
    let right = input.right.clone();

    let mut occurences: HashMap<usize, usize> = HashMap::new();

    for l in left.iter() {
        if !occurences.contains_key(l) {
            occurences.insert(*l, 0);
        } 
    }

    for r in right.iter() {
        if occurences.contains_key(r) {
            let entry = occurences.get_mut(r).unwrap();
            *entry += 1;
        } else {
            println!("{} not in occurences", r);
        }
    }

    occurences.iter().map(|(k, v)| k * v).sum()
}
