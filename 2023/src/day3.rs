use std::collections::HashMap;
use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[derive(Debug)]
pub struct Input {
    schematic: Vec<String>,
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Input {
    let lines = input.lines();

    let schematic = lines.map(|line| line.to_string()).collect::<Vec<String>>();
    
    Input { schematic }
}

#[aoc(day3, part1)]
pub fn part1(input: &Input) -> usize {
    let mut sum = 0;
    
    let re = Regex::new(r"\d+").unwrap();
    let schematic = input.schematic.clone();
    
    for (nr, line) in schematic.iter().enumerate() {
        let numbers = re.find_iter(&line).map(|m| {
            if has_symbol_neighbours(&schematic, m.start(), m.end(), nr) {
                m.as_str().parse::<usize>().ok().unwrap()
            } else {
                0
            }
        }).collect::<Vec<usize>>();
        sum += numbers.iter().sum::<usize>();
    }
    
    sum
}

#[aoc(day3, part2)]
pub fn part2(input: &Input) -> usize {
    let mut sum = 0;
    
    let re = Regex::new(r"\d+").unwrap();
    let schematic = input.schematic.clone();
    let mut gears: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
    
    for (nr, line) in schematic.iter().enumerate() {
        for m in re.find_iter(&line) {
            if let Some((line, pos)) = has_gear_neighbours(&schematic, m.start(), m.end(), nr) {
                gears.entry((line, pos)).or_insert(Vec::new()).push(m.as_str().parse::<usize>().ok().unwrap());
            }
        };
    }
    
    for gear in gears.values() {
        if gear.len() > 1 {
            sum += gear.iter().product::<usize>();
        }
    }
    
    sum
}

fn has_symbol_neighbours(schematic: &Vec<String>, start: usize, end: usize, line: usize) -> bool {
    let len = schematic[line].len();
    let re = Regex::new(r"[^\w.]").unwrap();
    
    //check if the character right before the match is a symbol
    if start > 0 {
        let before = schematic[line].chars().nth(start - 1).unwrap();
        if re.is_match(&before.to_string()) {
            return true;
        }
    }
    //check if the character right after the match is a symbol
    if end < len {
        let after = schematic[line].chars().nth(end).unwrap();
        if re.is_match(&after.to_string()) {
            return true;
        }
    }
    //check if the characters right above the match is a symbol
    if line > 0 {
        let above = &(schematic[line - 1][start..end]);
        if re.is_match(&above.to_string()) {
            return true;
        }
    }
    //check if the characters right below the match is a symbol
    if line < schematic.len() - 1 {
        let below = &(schematic[line + 1][start..end]);
        if re.is_match(&below.to_string()) {
            return true;
        }
    }
    //check if the characters in the diagonal corners are symbols
    if line > 0 && start > 0 {
        let above_left = schematic[line - 1].chars().nth(start - 1).unwrap();
        if re.is_match(&above_left.to_string()) {
            return true;
        }
    }
    if line > 0 && end < len {
        let above_right = schematic[line - 1].chars().nth(end).unwrap();
        if re.is_match(&above_right.to_string()) {
            return true;
        }
    }
    if line < schematic.len() - 1 && start > 0 {
        let below_left = schematic[line + 1].chars().nth(start - 1).unwrap();
        if re.is_match(&below_left.to_string()) {
            return true;
        }
    }
    if line < schematic.len() - 1 && end < len {
        let below_right = schematic[line + 1].chars().nth(end).unwrap();
        if re.is_match(&below_right.to_string()) {
            return true;
        }
    }
    
    false
}

fn has_gear_neighbours(schematic: &Vec<String>, start: usize, end: usize, line: usize) -> Option<(usize, usize)> {
    let len = schematic[line].len();
    let re = Regex::new(r"\*").unwrap();

    //check if the character right before the match is a symbol
    if start > 0 {
        let before = schematic[line].chars().nth(start - 1).unwrap();
        if re.is_match(&before.to_string()) {
            return Some((line, start - 1));
        }
    }
    //check if the character right after the match is a symbol
    if end < len {
        let after = schematic[line].chars().nth(end).unwrap();
        if re.is_match(&after.to_string()) {
            return Some((line, end));
        }
    }
    //check if the characters right above the match is a symbol
    if line > 0 {
        let above = &(schematic[line - 1][start..end]);
        for found in re.find(&above.to_string()) {
            return Some((line - 1, start + found.start()));
        }
    }
    //check if the characters right below the match is a symbol
    if line < schematic.len() - 1 {
        let below = &(schematic[line + 1][start..end]);
        for found in re.find(&below.to_string()) {
            return Some((line + 1, start + found.start()));
        }
    }
    //check if the characters in the diagonal corners are symbols
    if line > 0 && start > 0 {
        let above_left = schematic[line - 1].chars().nth(start - 1).unwrap();
        if re.is_match(&above_left.to_string()) {
            return Some((line - 1, start - 1));
        }
    }
    if line > 0 && end < len {
        let above_right = schematic[line - 1].chars().nth(end).unwrap();
        if re.is_match(&above_right.to_string()) {
            return Some((line - 1, end));
        }
    }
    if line < schematic.len() - 1 && start > 0 {
        let below_left = schematic[line + 1].chars().nth(start - 1).unwrap();
        if re.is_match(&below_left.to_string()) {
            return Some((line + 1, start - 1));
        }
    }
    if line < schematic.len() - 1 && end < len {
        let below_right = schematic[line + 1].chars().nth(end).unwrap();
        if re.is_match(&below_right.to_string()) {
            return Some((line + 1, end));
        }
    }

    None
}
