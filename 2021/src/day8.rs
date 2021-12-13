use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Debug)]
pub struct Entry {
    signal: Vec<String>,
    output: Vec<String>,
}

pub struct Input {
    entries: Vec<Entry>,
}

#[derive(Clone, Debug)]
pub struct Digit {
    segments: HashMap<char, usize>,
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Input {
    let lines = input.lines();

    let fish = lines
        .map(|x| {
            let mut parts = x.split("|");
            let signals = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|y| y.to_string())
                .collect::<Vec<String>>();
            let outputs = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|y| y.to_string())
                .collect::<Vec<String>>();
            Entry {
                signal: signals,
                output: outputs,
            }
        })
        .collect();

    Input { entries: fish }
}

#[aoc(day8, part1)]
pub fn part1(input: &Input) -> usize {
    input.entries.clone().into_iter().fold(0, |sum, x| {
        sum + x.output.into_iter().fold(0, |sum2, y| {
            sum2 + match y.len() {
                2 | 4 | 3 | 7 => 1,
                _ => 0,
            }
        })
    })
}

#[aoc(day8, part2)]
pub fn part2(input: &Input) -> usize {
    let mut entry_digits = vec![
        vec![
            Digit {
                segments: HashMap::new()
            };
            6
        ];
        input.entries.len()
    ];
    let mut output_digits = vec![
        vec![
            Digit {
                segments: HashMap::new()
            };
            6
        ];
        input.entries.len()
    ];

    // Count occurences of segments in digits with a certain number of segments
    for (i, entry) in input.entries.clone().into_iter().enumerate() {
        for x in entry.signal {
            let value = x.len() - 2;
            match value {
                0..=5 => {
                    for y in x.chars() {
                        let entry = entry_digits[i][value]
                            .segments
                            .entry(y)
                            .or_insert(0 as usize);
                        *entry = *entry + 1;
                    }
                }
                _ => (),
            }
        }
        for x in entry.output {
            let value = x.len() - 2;
            match value {
                0..=5 => {
                    for y in x.chars() {
                        let entry = output_digits[i][value]
                            .segments
                            .entry(y)
                            .or_insert(0 as usize);
                        *entry = *entry + 1;
                    }
                }
                _ => (),
            }
        }
    }

    // Create mapping of segments
    let mut segment_maps: Vec<HashMap<char, char>> = vec![HashMap::new(); input.entries.len()];
    (0..input.entries.len()).into_iter().fold(0, |sum, i| {
        // 1
        let f;
        let c;
        {
            let mut value = None;
            'outerfc: for character1 in entry_digits[i][0].segments.keys() {
                for (character2, occurences) in &entry_digits[i][4].segments {
                    if character1 == character2 && occurences == &2 {
                        value = Some(*character1);
                        break 'outerfc;
                    }
                }
            }

            if let Some(x) = value {
                c = x;
                segment_maps[i].insert(x, 'c');
                f = *entry_digits[i][0]
                    .segments
                    .keys()
                    .filter(|y| **y != x)
                    .collect::<Vec<&char>>()[0];
                segment_maps[i].insert(f, 'f');
            } else {
                unreachable!()
            }
        }
        // 7
        let a = *entry_digits[i][1]
            .segments
            .keys()
            .filter(|y| **y != f && **y != c)
            .collect::<Vec<&char>>()[0];
        segment_maps[i].insert(a, 'a');
        // 4
        let d;
        let b;
        {
            let mut value = None;
            'outerd: for character1 in entry_digits[i][2].segments.keys() {
                for (character2, occurences) in &entry_digits[i][3].segments {
                    if character1 == character2 && occurences == &3 {
                        value = Some(*character1);
                        break 'outerd;
                    }
                }
            }

            if let Some(x) = value {
                d = x;
                segment_maps[i].insert(d, 'd');
                b = *entry_digits[i][2]
                    .segments
                    .keys()
                    .filter(|y| **y != f && **y != c && **y != d)
                    .collect::<Vec<&char>>()[0];
                segment_maps[i].insert(b, 'b');
            } else {
                unreachable!()
            }
        }
        // 3
        let g = entry_digits[i][3]
            .clone()
            .segments
            .into_iter()
            .filter(|(y, z)| *y != a && *y != b && *y != c && *y != d && *y != f && z == &3)
            .map(|(x, _)| x)
            .collect::<Vec<char>>()[0];
        segment_maps[i].insert(g, 'g');
        // 8
        let e = *entry_digits[i][5]
            .segments
            .keys()
            .filter(|y| **y != a && **y != b && **y != c && **y != d && **y != f && **y != g)
            .collect::<Vec<&char>>()[0];
        segment_maps[i].insert(e, 'e');

        // No need to process more digits, we now know all 7 segments

        sum + input.entries[i]
            .clone()
            .output
            .into_iter()
            .map(|x| {
                let mut keys = x.chars().collect::<Vec<char>>();
                keys.sort();
                match keys.len() {
                    2 => '1',
                    3 => '7',
                    4 => '4',
                    7 => '8',
                    5 => {
                        if keys
                            .as_slice()
                            .into_iter()
                            .zip(&{
                                let mut slice = [a, c, d, e, g];
                                slice.sort();
                                slice
                            })
                            .filter(|(y, z)| y == z)
                            .count()
                            == 5
                        {
                            '2'
                        } else if keys
                            .as_slice()
                            .into_iter()
                            .zip(&{
                                let mut slice = [a, c, d, f, g];
                                slice.sort();
                                slice
                            })
                            .filter(|(y, z)| y == z)
                            .count()
                            == 5
                        {
                            '3'
                        } else if keys
                            .as_slice()
                            .into_iter()
                            .zip(&{
                                let mut slice = [a, b, d, f, g];
                                slice.sort();
                                slice
                            })
                            .filter(|(y, z)| y == z)
                            .count()
                            == 5
                        {
                            '5'
                        } else {
                            unreachable!()
                        }
                    }
                    6 => {
                        if keys
                            .as_slice()
                            .into_iter()
                            .zip(&{
                                let mut slice = [a, b, d, e, f, g];
                                slice.sort();
                                slice
                            })
                            .filter(|(y, z)| y == z)
                            .count()
                            == 6
                        {
                            '6'
                        } else if keys
                            .as_slice()
                            .into_iter()
                            .zip(&{
                                let mut slice = [a, b, c, d, f, g];
                                slice.sort();
                                slice
                            })
                            .filter(|(y, z)| y == z)
                            .count()
                            == 6
                        {
                            '9'
                        } else if keys
                            .as_slice()
                            .into_iter()
                            .zip(&{
                                let mut slice = [a, b, c, e, f, g];
                                slice.sort();
                                slice
                            })
                            .filter(|(y, z)| y == z)
                            .count()
                            == 6
                        {
                            '0'
                        } else {
                            unreachable!()
                        }
                    }
                    _ => unreachable!(),
                }
            })
            .collect::<String>()
            .parse::<usize>()
            .unwrap()
    })
}
