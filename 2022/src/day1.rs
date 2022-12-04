use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
pub struct Input {
    elves: Vec<Vec<usize>>,
}

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Input {
    let mut lines = input.lines();

    let mut elves = Vec::new();
    'elves: loop {
        let mut food = Vec::new();
        'foods: loop {
            let line = lines.next();
            if let Some(l) = line {
                if let "" = l {
                    break 'foods;
                } else {
                    food.push(l.parse::<usize>().unwrap())
                }
            } else {
                break 'elves;
            }
        }
        elves.push(food);
    }
    Input { elves: elves }
}

#[aoc(day1, part1)]
pub fn part1(input: &Input) -> usize {
    let mut max_calories = 0;

    for elf in input.elves.clone() {
        let sum: usize = elf.into_iter().sum();
        if sum > max_calories {
            max_calories = sum;
        }
    }

    max_calories
}

#[aoc(day1, part2)]
pub fn part2(input: &Input) -> usize {
    let mut top_1 = 0;
    let mut top_2 = 0;
    let mut top_3 = 0;

    for elf in input.elves.clone() {
        let sum: usize = elf.into_iter().sum();
        if sum > top_3 {
            if sum > top_2 {
                if sum > top_1 {
                    top_3 = top_2;
                    top_2 = top_1;
                    top_1 = sum;
                } else {
                    top_3 = top_2;
                    top_2 = sum;
                }
            } else {
                top_3 = sum;
            }
        }
    }

    top_1 + top_2 + top_3
}
