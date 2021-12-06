use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Copy)]
pub struct Fish {
    breeding_cooldown: usize,
}

impl Fish {
    fn new() -> Fish {
        Fish {
            breeding_cooldown: 8,
        }
    }

    fn new_day(&mut self) -> usize {
        match self.breeding_cooldown {
            0 => {
                self.breeding_cooldown = 6;
                1
            }
            other => {
                self.breeding_cooldown = other - 1;
                0
            }
        }
    }
}

pub struct NaiveInput {
    fish: Vec<Fish>,
}

pub struct CleverInput {
    ages: Vec<usize>,
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> NaiveInput {
    let mut lines = input.lines();

    let fish = lines
        .next()
        .unwrap()
        .split(",")
        .map(|x| Fish {
            breeding_cooldown: x.parse::<usize>().unwrap(),
        })
        .collect();

    NaiveInput { fish: fish }
}

pub fn input_generator_clever(input: &str) -> CleverInput {
    let mut lines = input.lines();

    let mut ages = vec![0; 9];
    for fish in lines.next().unwrap().split(",") {
        let i = fish.parse::<usize>().unwrap() - 1;
        ages[i] = ages[i] + 1
    }

    CleverInput { ages: ages }
}

#[aoc_generator(day6, part1, clever)]
pub fn input_generator_clever_part1(input: &str) -> CleverInput {
    input_generator_clever(input)
}

#[aoc_generator(day6, part2, clever)]
pub fn input_generator_clever_part2(input: &str) -> CleverInput {
    input_generator_clever(input)
}

fn number_of_fish(mut fish: Vec<Fish>, days: usize) -> usize {
    for _ in 0..days {
        let mut new_fish = 0;
        for i in 0..fish.len() {
            new_fish = new_fish + fish[i].new_day();
        }
        for _ in 0..new_fish {
            fish.push(Fish::new());
        }
    }
    fish.len()
}

fn number_of_fish_one_loop(mut fish: Vec<Fish>, days: usize) -> usize {
    for _ in 0..days {
        for i in 0..fish.len() {
            match fish[i].new_day() {
                1 => fish.push(Fish::new()),
                _ => (),
            }
        }
    }

    fish.len()
}

fn number_of_fish_clever(mut ages: Vec<usize>, days: usize) -> usize {
    for _ in 1..days {
        let new_fish;
        new_fish = ages[0];
        ages[0] = ages[1];
        ages[1] = ages[2];
        ages[2] = ages[3];
        ages[3] = ages[4];
        ages[4] = ages[5];
        ages[5] = ages[6];
        ages[6] = ages[7] + new_fish;
        ages[7] = ages[8];
        ages[8] = new_fish;
    }
    println!("{:?}", ages);

    ages.into_iter().fold(0, |sum, x| sum + x)
}

#[aoc(day6, part1, naive_separate)]
pub fn part1(input: &NaiveInput) -> usize {
    number_of_fish(input.fish.clone(), 80)
}

#[aoc(day6, part1, naive_combined)]
pub fn part1_combined(input: &NaiveInput) -> usize {
    number_of_fish_one_loop(input.fish.clone(), 80)
}

#[aoc(day6, part1, clever)]
pub fn part1_clever(input: &CleverInput) -> usize {
    number_of_fish_clever(input.ages.clone(), 80)
}

#[aoc(day6, part2, clever)]
pub fn part2_clever(input: &CleverInput) -> usize {
    number_of_fish_clever(input.ages.clone(), 256)
}

#[aoc(day6, part2, naive)]
pub fn part2(input: &NaiveInput) -> usize {
    number_of_fish_one_loop(input.fish.clone(), 256)
}
