use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

pub struct Input {
    rows: Vec<Vec<usize>>,
    width: usize,
    height: usize,
}

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Input {
    let lines = input.lines();

    let rows: Vec<Vec<usize>> = lines
        .map(|x| {
            x.chars()
                .map(|y| String::from(y).parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    Input {
        width: rows[0].len(),
        height: rows.len(),
        rows: rows,
    }
}

#[aoc(day9, part1)]
pub fn part1(input: &Input) -> usize {
    (0..input.height).fold(0, |sum, y| {
        sum + (0..input.width).fold(0, |sum2, x| sum2 + low_point_score(input, y, x))
    })
}

pub fn low_point_score(input: &Input, y: usize, x: usize) -> usize {
    let value = input.rows[y][x];
    if y != 0 && input.rows[y - 1][x] <= value {
        return 0;
    };
    if y != input.height - 1 && input.rows[y + 1][x] <= value {
        return 0;
    }
    if x != 0 && input.rows[y][x - 1] <= value {
        return 0;
    };
    if x != input.width - 1 && input.rows[y][x + 1] <= value {
        return 0;
    }
    value + 1
}

#[aoc(day9, part2)]
pub fn part2(input: &Input) -> usize {
    let mut basins: Vec<Vec<Option<usize>>> = vec![vec![None; input.width]; input.height];
    let mut basin_count = 0;
    // Find all possible basins, even if split
    for y in 0..input.height {
        for x in 0..input.width {
            (basins[y][x], basin_count) = find_basins(input, y, x, &basins, basin_count);
        }
    }
    // Merge adjecent basins
    for y in 0..input.height {
        for x in 0..input.width {
            basins = merge_basins(input, y, x, basins);
        }
    }
    // Find largest basins
    let mut basin_sizes: HashMap<usize, usize> = HashMap::new();
    for row in basins {
        for entry in row {
            if let Some(entry) = entry {
                let entry = basin_sizes.entry(entry).or_insert(0 as usize);
                *entry = *entry + 1;
            }
        }
    }
    let mut basin_sizes: Vec<(usize, usize)> = basin_sizes.into_iter().collect();
    basin_sizes.sort_by(|(_, x), (_, y)| y.cmp(x));
    basin_sizes[0].1 * basin_sizes[1].1 * basin_sizes[2].1
}

pub fn merge_basins(
    input: &Input,
    y: usize,
    x: usize,
    basins: Vec<Vec<Option<usize>>>,
) -> Vec<Vec<Option<usize>>> {
    let mut basins = basins.clone();
    let value = basins[y][x];
    if let Some(value) = value {
        if y != 0 && basins[y - 1][x] != None && basins[y - 1][x] != Some(value) {
            basins = exchange_basins(
                basins[y - 1][x].unwrap(),
                value,
                basins,
                input.height,
                input.width,
            );
        };
        if y != input.height - 1 && basins[y + 1][x] != None && basins[y + 1][x] != Some(value) {
            basins = exchange_basins(
                basins[y + 1][x].unwrap(),
                value,
                basins,
                input.height,
                input.width,
            );
        }
        if x != 0 && basins[y][x - 1] != None && basins[y][x - 1] != Some(value) {
            basins = exchange_basins(
                basins[y][x - 1].unwrap(),
                value,
                basins,
                input.height,
                input.width,
            );
        };
        if x != input.width - 1 && basins[y][x + 1] != None && basins[y][x + 1] != Some(value) {
            basins = exchange_basins(
                basins[y][x + 1].unwrap(),
                value,
                basins,
                input.height,
                input.width,
            );
        };
    }
    basins
}

pub fn exchange_basins(
    old: usize,
    new: usize,
    basins: Vec<Vec<Option<usize>>>,
    y: usize,
    x: usize,
) -> Vec<Vec<Option<usize>>> {
    let mut basins = basins.clone();
    for y in 0..y {
        for x in 0..x {
            if basins[y][x] == Some(old) {
                basins[y][x] = Some(new)
            }
        }
    }
    basins
}

pub fn find_basins(
    input: &Input,
    y: usize,
    x: usize,
    basins: &Vec<Vec<Option<usize>>>,
    basin_count: usize,
) -> (Option<usize>, usize) {
    if input.rows[y][x] == 9 {
        return (None, basin_count);
    }
    if y != 0 && input.rows[y - 1][x] != 9 {
        return (basins[y - 1][x], basin_count);
    };
    if x != 0 && input.rows[y][x - 1] != 9 {
        return (basins[y][x - 1], basin_count);
    };
    (Some(basin_count), basin_count + 1)
}
