use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CaveSize {
    Small,
    Large,
}

#[derive(Clone, Debug)]
pub struct Cave {
    connections: Vec<String>,
    size: CaveSize,
    visited: usize,
}

pub struct Input {
    caves: HashMap<String, Cave>,
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Input {
    let lines = input.lines();
    let mut caves = HashMap::new();

    for line in lines {
        let mut connection = line.split("-");
        let cave0 = connection.next().unwrap().to_string();
        let cave1 = connection.next().unwrap().to_string();
        {
            let entry = caves.entry(cave0.clone()).or_insert(Cave {
                connections: Vec::new(),
                size: match cave0.clone().chars().next().unwrap().is_uppercase() {
                    true => CaveSize::Large,
                    false => CaveSize::Small,
                },
                visited: 0,
            });
            (*entry).connections.push(cave1.clone());
        }
        {
            let entry = caves.entry(cave1.clone()).or_insert(Cave {
                connections: Vec::new(),
                size: match cave1.clone().chars().next().unwrap().is_uppercase() {
                    true => CaveSize::Large,
                    false => CaveSize::Small,
                },
                visited: 0,
            });
            (*entry).connections.push(cave0);
        }
    }

    Input { caves: caves }
}

#[aoc(day12, part1)]
pub fn part1(input: &Input) -> usize {
    process_cave_part1(input.caves.clone(), "start".to_string())
}

pub fn process_cave_part1(mut caves: HashMap<String, Cave>, current_cave: String) -> usize {
    let mut found_routes_from_here = 0;

    // We found another way to the exit
    if current_cave == "end" {
        return 1;
    }

    // Mark that we've now been in this cave
    let entry = caves.get_mut(&current_cave).unwrap();
    (*entry).visited = (*entry).visited + 1;

    // Search connections for more ways out
    for cave_name in caves.get(&current_cave).unwrap().connections.clone() {
        let cave = caves.get(&cave_name).unwrap();
        if cave_name == "end"
            || cave.size == CaveSize::Large
            || ((cave.visited < 1) && cave.size == CaveSize::Small)
        {
            found_routes_from_here =
                found_routes_from_here + process_cave_part1(caves.clone(), cave_name)
        }
    }
    found_routes_from_here
}

#[aoc(day12, part2)]
pub fn part2(input: &Input) -> usize {
    process_cave_part2(input.caves.clone(), "start".to_string(), false)
}

pub fn process_cave_part2(
    mut caves: HashMap<String, Cave>,
    current_cave: String,
    mut has_visited_a_small_cave_twice: bool,
) -> usize {
    let mut found_routes_from_here = 0;

    // We found another way to the exit
    if current_cave == "end" {
        return 1;
    }

    // Mark that we've now been in this cave
    let entry = caves.get_mut(&current_cave).unwrap();
    (*entry).visited = (*entry).visited + 1;

    // Mark if this is a small cave and the second time we've been in it
    if caves.get(&current_cave).unwrap().size == CaveSize::Small
        && caves.get(&current_cave).unwrap().visited == 2
    {
        has_visited_a_small_cave_twice = true
    }

    // Search connections for more ways out
    for cave_name in caves.get(&current_cave).unwrap().connections.clone() {
        let cave = caves.get(&cave_name).unwrap();
        if cave_name != "start"
            && (cave_name == "end"
                || cave.size == CaveSize::Large
                || (cave.size == CaveSize::Small
                    && ((cave.visited < 2 && !has_visited_a_small_cave_twice) || cave.visited < 1)))
        {
            found_routes_from_here = found_routes_from_here
                + process_cave_part2(caves.clone(), cave_name, has_visited_a_small_cave_twice)
        }
    }
    found_routes_from_here
}
