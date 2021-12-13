use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Copy)]
pub struct CrabMarine {
    position: usize,
}

pub struct Input {
    crabs: Vec<CrabMarine>,
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Input {
    let mut lines = input.lines();

    let fish = lines
        .next()
        .unwrap()
        .split(",")
        .map(|x| CrabMarine {
            position: x.parse::<usize>().unwrap(),
        })
        .collect();

    Input { crabs: fish }
}

#[aoc(day7, part1)]
pub fn part1(input: &Input) -> usize {
    let mut values = input
        .crabs
        .clone()
        .into_iter()
        .map(|x| x.position)
        .collect::<Vec<usize>>();
    values.sort();
    let target = values[input.crabs.len() / 2];
    input
        .crabs
        .clone()
        .into_iter()
        .map(|x| x.position.abs_diff(target))
        .sum::<usize>()
}

#[aoc(day7, part2)]
pub fn part2(input: &Input) -> usize {
    let target: usize = input
        .crabs
        .clone()
        .into_iter()
        .map(|x| x.position)
        .sum::<usize>()
        / input.crabs.len();
    input
        .crabs
        .clone()
        .into_iter()
        .map(|x| x.position.abs_diff(target))
        .map(|x| (x * (x + 1)) / 2)
        .sum::<usize>()
}
