use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};


#[derive(PartialEq, Clone, Eq, Hash)]
pub struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(Clone)]
pub struct Vector {
    start: Coordinate,
    stop: Coordinate,
}

pub struct Input {
    vectors: Vec<Vector>,
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Input {
    let lines = input.lines();

    let vectors = lines
        .map(|x| {
            let mut parts = x.split(" -> ");
            let mut start = parts.next().unwrap().split(",");
            let mut stop = parts.next().unwrap().split(",");
            let x1 = start.next().unwrap().parse::<usize>().unwrap();
            let y1 = start.next().unwrap().parse::<usize>().unwrap();
            let x2 = stop.next().unwrap().parse::<usize>().unwrap();
            let y2 = stop.next().unwrap().parse::<usize>().unwrap();
            Vector {
                start: Coordinate { x: x1, y: y1 },
                stop: Coordinate { x: x2, y: y2 },
            }
        })
        .collect::<Vec<Vector>>();

    Input { vectors: vectors }
}


#[aoc(day5, part1)]
pub fn part1_map(input: &Input) -> usize {
    let all_points = input.vectors.clone().into_iter().map(|x| generate_points(&x, false)).collect::<Vec<Vec<Coordinate>>>().concat();
    let mut map = HashMap::new();
    for point in all_points {
        let entry = map.entry(point).or_insert(0 as usize);
        *entry = *entry + 1;
    }
    map.into_iter().map(|(_, x)| x).filter(|x| x > &1).collect::<Vec<usize>>().len()
}

#[aoc(day5, part2)]
pub fn part2_map(input: &Input) -> usize {
    let all_points = input.vectors.clone().into_iter().map(|x| generate_points(&x, true)).collect::<Vec<Vec<Coordinate>>>().concat();
    let mut map = HashMap::new();
    for point in all_points {
        let entry = map.entry(point).or_insert(0 as usize);
        *entry = *entry + 1;
    }
    map.into_iter().map(|(_, x)| x).filter(|x| x > &1).collect::<Vec<usize>>().len()
}

fn generate_points(vector: &Vector, diagonal: bool) -> Vec<Coordinate> {
    let v1x: Vec<usize> = if vector.start.x <= vector.stop.x {
        (vector.start.x..=vector.stop.x).collect()
    } else {
        (vector.stop.x..=vector.start.x).rev().collect()
    };
    let v1y: Vec<usize> = if vector.start.y <= vector.stop.y {
        (vector.start.y..=vector.stop.y).collect()
    } else {
        (vector.stop.y..=vector.start.y).rev().collect()
    };

    let v1p: Vec<Coordinate> = if v1x.len() == 1  {
        v1y.into_iter().map(|y| Coordinate { x: v1x[0], y: y }).collect()
    } else if v1y.len() == 1{
        v1x.into_iter().map(|x| Coordinate { x: x, y: v1y[0] }).collect()
    } else {
        if diagonal {
            (v1x.into_iter().zip(v1y)).map(|(x, y)| Coordinate { x: x, y: y}).collect()
        } else {
            Vec::new()
        }
    };

    v1p
}
