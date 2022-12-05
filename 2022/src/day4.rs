use aoc_runner_derive::{aoc, aoc_generator};
use gcollections::ops::set::{Overlap, Subset};
use interval::{ops::Range, Interval};

#[derive(Debug, Clone)]
pub struct Input {
    pairs: Vec<(Interval<usize>, Interval<usize>)>,
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Input {
    let pairs = input
        .lines()
        .map(|line| {
            let pairs = line.split(',').collect::<Vec<&str>>();
            assert_eq!(pairs.len(), 2);
            let (elf_1, elf_2) = (
                pairs[0]
                    .split('-')
                    .into_iter()
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>(),
                pairs[1]
                    .split('-')
                    .into_iter()
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>(),
            );
            assert_eq!(elf_1.len(), 2);
            assert_eq!(elf_2.len(), 2);
            let elf_1 = Interval::new(elf_1[0], elf_1[1]);
            let elf_2 = Interval::new(elf_2[0], elf_2[1]);
            (elf_1, elf_2)
        })
        .collect();
    Input { pairs: pairs }
}

#[aoc(day4, part1)]
pub fn part1(input: &Input) -> usize {
    input
        .pairs
        .clone()
        .into_iter()
        .map(|pair| {
            if pair.0.is_subset(&pair.1) {
                1
            } else if pair.1.is_subset(&pair.0) {
                1
            } else {
                0
            }
        })
        .sum()
}

#[aoc(day4, part2)]
pub fn part2(input: &Input) -> usize {
    input
        .pairs
        .clone()
        .into_iter()
        .map(|pair| if pair.0.overlap(&pair.1) { 1 } else { 0 })
        .sum()
}
