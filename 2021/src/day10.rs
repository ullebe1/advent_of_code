use aoc_runner_derive::{aoc, aoc_generator};

pub struct Input {
    rows: Vec<String>,
}

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Input {
    let lines = input.lines().into_iter().map(|x| x.to_string()).collect();
    Input { rows: lines }
}

#[aoc(day10, part1)]
pub fn part1(input: &Input) -> usize {
    input
        .rows
        .clone()
        .into_iter()
        .fold(0, |sum, x| sum + handle_line_corrupted(x))
}

#[aoc(day10, part2)]
pub fn part2(input: &Input) -> usize {
    let mut scores = input
        .rows
        .clone()
        .into_iter()
        .map(|x| handle_line_discarded(x))
        .filter(|x| x != &0)
        .collect::<Vec<usize>>();
    scores.sort();
    scores[scores.len() / 2]
}

pub fn handle_line_corrupted(x: String) -> usize {
    let mut stack: Vec<char> = Vec::new();
    for char in x.chars() {
        match char {
            '(' => stack.push(char),
            ')' => {
                if let Some(x) = stack.pop() {
                    if x != '(' {
                        return 3;
                    }
                } else {
                    return 0;
                }
            }
            '[' => stack.push(char),
            ']' => {
                if let Some(x) = stack.pop() {
                    if x != '[' {
                        return 57;
                    }
                } else {
                    return 0;
                }
            }
            '{' => stack.push(char),
            '}' => {
                if let Some(x) = stack.pop() {
                    if x != '{' {
                        return 1197;
                    }
                } else {
                    return 0;
                }
            }
            '<' => stack.push(char),
            '>' => {
                if let Some(x) = stack.pop() {
                    if x != '<' {
                        return 25137;
                    }
                } else {
                    return 0;
                }
            }
            _ => unreachable!(),
        }
    }

    0
}

pub fn handle_line_discarded(x: String) -> usize {
    let mut stack: Vec<char> = Vec::new();
    for char in x.chars() {
        match char {
            '(' => stack.push(char),
            ')' => {
                if let Some(x) = stack.pop() {
                    if x != '(' {
                        return 0;
                    }
                }
            }
            '[' => stack.push(char),
            ']' => {
                if let Some(x) = stack.pop() {
                    if x != '[' {
                        return 0;
                    }
                }
            }
            '{' => stack.push(char),
            '}' => {
                if let Some(x) = stack.pop() {
                    if x != '{' {
                        return 0;
                    }
                }
            }
            '<' => stack.push(char),
            '>' => {
                if let Some(x) = stack.pop() {
                    if x != '<' {
                        return 0;
                    }
                }
            }
            _ => unreachable!(),
        }
    }
    stack.reverse();
    let mut score = 0;
    for char in stack {
        match char {
            '(' => score = (score * 5) + 1,
            '[' => score = (score * 5) + 2,
            '{' => score = (score * 5) + 3,
            '<' => score = (score * 5) + 4,
            _ => unreachable!(),
        }
    }

    score
}
