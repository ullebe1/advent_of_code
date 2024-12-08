use std::collections::{HashMap, VecDeque};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
pub struct Input {
    rules: Vec<(usize, usize)>,
    updates: Vec<Vec<usize>>,
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Input {
    let mut lines = input.lines();
    let mut rules_lines = Vec::new();
    let mut updates_lines = Vec::new();

    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }
        rules_lines.push(line);
    }
    loop {
        let line = lines.next();
        if line.is_none() {
            break;
        }
        updates_lines.push(line.unwrap());
    }

    let rules = rules_lines.iter().map(|line| {
        let mut parts = line.split("|");
        let before = parts.next().unwrap().parse::<usize>().unwrap();
        let after = parts.next().unwrap().parse::<usize>().unwrap();

        (before, after)
    }).collect();

    let updates = updates_lines.iter().map(|line| {
        line.split(",").map(|s| s.parse::<usize>().unwrap()).collect()
    }).collect::<Vec<Vec<usize>>>();

    Input { rules, updates }
}

#[aoc(day5, part1)]
pub fn part1(input: &Input) -> usize {
    let mut valid_updates = Vec::new();

    for update in input.updates.iter() {
        let mut valid = true;
        let mut printed_pages = Vec::new();

        for page in update.iter() {
            for rule in input.rules.iter() {
                if rule.0 == *page {
                    if printed_pages.contains(&rule.1) {
                        valid = false;
                        break;
                    }
                }
            }
            printed_pages.push(*page);
        }

        if valid {
            valid_updates.push(update);
        }
    }

    valid_updates.iter().map(|update| update[update.len()/2]).sum()
}

#[aoc(day5, part2)]
pub fn part2(input: &Input) -> usize {
    let mut invalid_updates = Vec::new();

    for update in input.updates.iter() {
        let mut valid = true;
        let mut printed_pages = Vec::new();

        for page in update.iter() {
            for rule in input.rules.iter() {
                if rule.0 == *page {
                    if printed_pages.contains(&rule.1) {
                        valid = false;
                        break;
                    }
                }
            }
            printed_pages.push(*page);
        }

        if !valid {
            invalid_updates.push(update);
        }
    }

    let mut rules: HashMap<usize, Vec<usize>> = HashMap::new();

    for rule in input.rules.iter() {
        if rules.contains_key(&rule.0) {
            let pages = rules.get_mut(&rule.0).unwrap();
            pages.push(rule.1);
        } else {
            rules.insert(rule.0, vec![rule.1]);
        }
    }

    let invalid_updates = invalid_updates.iter().map(|update| {
        let mut printed_pages = Vec::new();
        let mut pages: VecDeque<usize> = (*update).clone().into();

        while let Some(page) = pages.pop_front() {
            if rules.contains_key(&page) {
                let comes_after = rules.get(&page).unwrap();
                let mut intersection = false;
                for page in comes_after.iter() {
                    if pages.contains(page) {
                        intersection = true;
                    }
                }

                if intersection {
                    pages.push_back(page);
                } else {
                    let page = page.clone();
                    printed_pages.push(page);
                }
            } else {
                let page = page.clone();
                printed_pages.push(page);
            }
        }

        printed_pages
    }).collect::<Vec<Vec<usize>>>();
    
    invalid_updates.iter().map(|update| update[update.len()/2]).sum()
}