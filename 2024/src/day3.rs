use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[derive(Debug)]
pub struct Input {
    memory: Vec<String>,
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Input {
    let lines = input.lines();

    let memory = lines.map(|x| x.to_owned()).collect::<Vec<String>>();

    Input { memory }
}

#[aoc(day3, part1)]
pub fn part1(input: &Input) -> usize {
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();

    let mut sum = 0;

    for line in input.memory.iter() {
        sum += re.captures_iter(line).map(|capture| {
            let capture = capture.get(0).unwrap();
            let mut capture = capture.as_str()[4..].chars();
            capture.next_back();
            let capture = capture.as_str();
            let values: Vec<&str> = capture.split(',').collect();
            let a: usize = values[0].parse().unwrap();
            let b: usize = values[1].parse().unwrap();

            a * b
        }).sum::<usize>();
    }

    sum
}

#[aoc(day3, part2)]
pub fn part2(input: &Input) -> usize {
    let re = Regex::new(r"(mul\(\d{1,3},\d{1,3}\))|(do\(\))|(don't\(\))").unwrap();

    let mut sum = 0;
    let mut enabled = true;

    for line in input.memory.iter() {

        sum += re.captures_iter(line).map(|capture| {
            let capture = capture.get(0).unwrap();
            let capture = capture.as_str();

            match capture {
                s if s.starts_with("mul(") => {
                    if enabled {
                        let mut capture = capture[4..].chars();
                        capture.next_back();
                        let capture = capture.as_str();
                        let values: Vec<&str> = capture.split(',').collect();
                        let a: usize = values[0].parse().unwrap();
                        let b: usize = values[1].parse().unwrap();

                        return a * b;
                    } else {
                        return 0;
                    }
                },
                s if s.starts_with("do(") => {enabled = true; return 0},
                s if s.starts_with("don't(") => {enabled = false; return 0},
                _ => panic!("Unknown prefix"),
                
            }
        }).sum::<usize>();
    }

    sum
}