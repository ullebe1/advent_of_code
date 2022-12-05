use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct Instruction {
    from: usize,
    to: usize,
    number: usize,
}

#[derive(Debug, Clone)]
pub struct Input {
    stacks: Vec<Vec<char>>,
    instructions: Vec<Instruction>,
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Input {
    let lines = input.lines().collect::<Vec<&str>>();
    let lines = lines.as_slice().split(|x| x == &"").collect::<Vec<_>>();
    assert_eq!(lines.len(), 2);
    // Parse intial stacks
    let number_of_stacks = ((lines[0][0].len() - 3) / 4) + 1;
    let mut stacks = vec![Vec::<char>::new(); number_of_stacks];
    for &line in lines[0] {
        let line_stacks = line
            .chars()
            .chunks(4)
            .into_iter()
            .map(|mut chunk| chunk.find(|x| x.is_alphabetic()))
            .enumerate()
            .collect::<Vec<_>>();
        for slot in line_stacks {
            if let Some(container) = slot.1 {
                stacks[slot.0].push(container)
            }
        }
    }
    // Reverse stacks so we can push and pop them when executing the instructions
    let stacks = stacks
        .into_iter()
        .map(|mut stack| {
            stack.reverse();
            stack
        })
        .collect();

    // Parse instructions
    let instruction_regex = Regex::new(r"\d+").unwrap();
    let instructions = lines[1]
        .into_iter()
        .map(|line| {
            let numbers = instruction_regex
                .find_iter(line)
                .map(|capture| capture.as_str().parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            assert_eq!(numbers.len(), 3);
            Instruction {
                from: numbers[1],
                to: numbers[2],
                number: numbers[0],
            }
        })
        .collect();
    Input {
        stacks: stacks,
        instructions: instructions,
    }
}

#[aoc(day5, part1)]
pub fn part1(input: &Input) -> String {
    let mut stacks = input.stacks.clone();
    for instruction in input.instructions.clone() {
        if instruction.from < instruction.to {
            let (first, second) = stacks.split_at_mut(instruction.to - 1);
            let to = &mut (second[0]);
            let from = &mut (first[instruction.from - 1]);
            for _ in 0..instruction.number {
                to.push(from.pop().unwrap());
            }
        }
        if instruction.from > instruction.to {
            let (first, second) = stacks.split_at_mut(instruction.from - 1);
            let from = &mut (second[0]);
            let to = &mut (first[instruction.to - 1]);
            for _ in 0..instruction.number {
                to.push(from.pop().unwrap());
            }
        }
    }
    let mut output = String::new();
    for mut stack in stacks {
        output.push(stack.pop().unwrap())
    }
    output
}

#[aoc(day5, part2)]
pub fn part2(input: &Input) -> String {
    let mut stacks = input.stacks.clone();
    for instruction in input.instructions.clone() {
        let mut lifted_stack: Vec<char> = Vec::new();
        if instruction.from < instruction.to {
            let (first, second) = stacks.split_at_mut(instruction.to - 1);
            let to = &mut (second[0]);
            let from = &mut (first[instruction.from - 1]);
            for _ in 0..instruction.number {
                lifted_stack.push(from.pop().unwrap());
            }
            for _ in 0..instruction.number {
                to.push(lifted_stack.pop().unwrap());
            }
        }
        if instruction.from > instruction.to {
            let (first, second) = stacks.split_at_mut(instruction.from - 1);
            let from = &mut (second[0]);
            let to = &mut (first[instruction.to - 1]);
            for _ in 0..instruction.number {
                lifted_stack.push(from.pop().unwrap());
            }
            for _ in 0..instruction.number {
                to.push(lifted_stack.pop().unwrap());
            }
        }
    }
    let mut output = String::new();
    for mut stack in stacks {
        output.push(stack.pop().unwrap())
    }
    output
}
