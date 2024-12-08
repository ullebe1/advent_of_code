use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[derive(Debug)]
pub struct Input {
    map: Vec<Vec<Option<char>>>,
    frequencies: HashSet<char>,
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Input {
    let mut frequencies = HashSet::new();
    let map = input.lines().map(|line| {
        line.chars().map(|c| {
            match c {
                '.' => None,
                character => {
                    frequencies.insert(character);
                    Some(character)
                },
            }
        }).collect()
    }).collect();

    Input { map, frequencies }
}

#[aoc(day8, part1)]
pub fn part1(input: &Input) -> usize {
    let heigth = input.map.len();
    let width = input.map[0].len();
    let mut all_antinodes: HashSet<(usize, usize)> = HashSet::new();

    for frequency in input.frequencies.iter() {
        let mut locations: Vec<(usize, usize)> = Vec::new();
        let mut antinodes: Vec<(usize, usize)> = Vec::new();

        for (i, row) in input.map.iter().enumerate() {
            for (j, character) in row.iter().enumerate() {
                if let Some(c) = character {
                    if c == frequency {
                        locations.push((i, j));
                    }
                }
            }
        }

        for location in locations.iter() {
            for location_two in locations.iter() {
                if location != location_two {
                    let x_offset = location.0 as isize - location_two.0 as isize;
                    let y_offset = location.1 as isize - location_two.1 as isize;

                    let new_antinode: (isize, isize) = (location.0 as isize + x_offset, location.1 as isize + y_offset);

                    if new_antinode.0 >= 0 && new_antinode.0 < heigth as isize && new_antinode.1 >= 0 && new_antinode.1 < width as isize {
                        antinodes.push((new_antinode.0 as usize, new_antinode.1 as usize));
                    }
                }
            }
        }

        for antinode in antinodes.iter() {
            all_antinodes.insert(*antinode);
        }
    }


    let mut new_map = input.map.clone();

    for antinode in all_antinodes.iter() {
        new_map[antinode.0][antinode.1] = Some('#');
    }

    for row in new_map.iter() {
        for character in row.iter() {
            if let Some(c) = character {
                print!("{}", c);
            } else {
                print!(".");
            }
        }
        println!();
    }

    all_antinodes.len()
}

#[aoc(day8, part2)]
pub fn part2(input: &Input) -> usize {
    let heigth = input.map.len();
    let width = input.map[0].len();
    let mut all_antinodes: HashSet<(usize, usize)> = HashSet::new();

    for frequency in input.frequencies.iter() {
        let mut locations: Vec<(usize, usize)> = Vec::new();
        let mut antinodes: Vec<(usize, usize)> = Vec::new();

        for (i, row) in input.map.iter().enumerate() {
            for (j, character) in row.iter().enumerate() {
                if let Some(c) = character {
                    if c == frequency {
                        locations.push((i, j));
                    }
                }
            }
        }

        for location in locations.iter() {
            for location_two in locations.iter() {
                if location != location_two {
                    let x_offset = location.0 as isize - location_two.0 as isize;
                    let y_offset = location.1 as isize - location_two.1 as isize;

                    let mut multiplier = 1;
                    loop {
                        let new_antinode: (isize, isize) = (location.0 as isize + (multiplier * x_offset), location.1 as isize + (multiplier* y_offset));

                        if new_antinode.0 >= 0 && new_antinode.0 < heigth as isize && new_antinode.1 >= 0 && new_antinode.1 < width as isize {
                            antinodes.push((new_antinode.0 as usize, new_antinode.1 as usize));
                            multiplier += 1;
                        } else {
                            break;
                        }
                    }

                    antinodes.push((location_two.0, location_two.1));
                }
            }
        }

        for antinode in antinodes.iter() {
            all_antinodes.insert(*antinode);
        }
    }


    let mut new_map = input.map.clone();

    for antinode in all_antinodes.iter() {
        new_map[antinode.0][antinode.1] = Some('#');
    }

    for row in new_map.iter() {
        for character in row.iter() {
            if let Some(c) = character {
                print!("{}", c);
            } else {
                print!(".");
            }
        }
        println!();
    }

    all_antinodes.len()
}