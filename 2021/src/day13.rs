use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Debug)]
pub enum Axis {
    X,
    Y,
}

#[derive(Clone, Debug)]
pub struct Fold {
    axis: Axis,
}

pub struct Input {
    width: usize,
    height: usize,
    rows: Vec<Vec<bool>>,
    instructions: Vec<Fold>,
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Input {
    let coordinates = input
        .clone()
        .lines()
        .filter(|x| match x.chars().next() {
            Some(y) => y.is_numeric(),
            None => false,
        })
        .map(|x| {
            let mut pieces = x.split(",");
            (
                pieces.next().unwrap().parse::<usize>().unwrap(),
                pieces.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<(usize, usize)>>();
    let height = coordinates
        .clone()
        .into_iter()
        .max_by(|x, y| x.1.cmp(&y.1))
        .unwrap()
        .1
        + 1;
    let width = coordinates
        .clone()
        .into_iter()
        .max_by(|x, y| x.0.cmp(&y.0))
        .unwrap()
        .0
        + 1;
    let mut rows = vec![vec![false; width]; height];

    for line in coordinates {
        rows[line.1][line.0] = true
    }

    let instructions = input
        .lines()
        .filter(|x| match x.chars().next() {
            Some(y) => y.is_alphabetic(),
            None => false,
        })
        .map(|x| Fold {
            axis: match &x[11..12] {
                "x" => Axis::X,
                "y" => Axis::Y,
                _ => unreachable!(),
            },
        })
        .collect();

    println!("{},{}", height, width);

    Input {
        width: width,
        height: height,
        rows: rows,
        instructions: instructions,
    }
}

#[aoc(day13, part1)]
pub fn part1(input: &Input) -> usize {
    let mut rows = input.rows.clone();
    let folds = input.instructions.clone();
    let mut height = input.height.clone();
    let mut width = input.width.clone();

    for fold in folds[0..1].to_vec() {
        (rows, height, width) = folder(rows, fold, height, width);
    }

    rows.into_iter().fold(0, |sum, x| {
        sum + x.into_iter().fold(0, |sum, y| sum + if y { 1 } else { 0 })
    })
}

#[aoc(day13, part2)]
pub fn part2(input: &Input) -> usize {
    let mut rows = input.rows.clone();
    let folds = input.instructions.clone();
    let mut height = input.height.clone();
    let mut width = input.width.clone();

    for row in rows.clone() {
        for value in row {
            print!("{}", if value { "#" } else { "." });
        }
        print!("\n");
    }
    println!("");

    for (i, fold) in folds.into_iter().enumerate() {
        println!("{}, {}, {}, {:?}", i, height, width, fold);
        (rows, height, width) = folder(rows, fold, height, width);
        for row in rows.clone() {
            for value in row {
                print!("{}", if value { "#" } else { "." });
            }
            print!("\n");
        }
        println!("");
    }

    rows.into_iter().fold(0, |sum, x| {
        sum + x.into_iter().fold(0, |sum, y| sum + if y { 1 } else { 0 })
    })
}

pub fn folder(
    mut rows: Vec<Vec<bool>>,
    fold: Fold,
    mut height: usize,
    mut width: usize,
) -> (Vec<Vec<bool>>, usize, usize) {
    match fold.axis {
        Axis::X => {
            width = width / 2;
            let mut new_rows = vec![vec![false; width]; height];
            let left_columns: Vec<Vec<bool>> = rows
                .clone()
                .into_iter()
                .map(|x| x[..width].to_vec())
                .collect();
            rows = rows
                .into_iter()
                .map(|x| x.into_iter().rev().collect())
                .collect();
            let right_columns: Vec<Vec<bool>> = rows
                .clone()
                .into_iter()
                .map(|x| x[..width].to_vec())
                .collect();
            assert_eq!(left_columns[0].len(), right_columns[0].len());
            assert_eq!(left_columns[0].len(), width);

            for (y, (left, right)) in left_columns
                .into_iter()
                .zip(right_columns.into_iter())
                .enumerate()
            {
                for (x, (left, right)) in left.into_iter().zip(right.into_iter()).enumerate() {
                    new_rows[y][x] = left || right
                }
            }
            rows = new_rows
        }
        Axis::Y => {
            height = height / 2;
            let mut new_rows = vec![vec![false; width]; height];
            let top_rows: Vec<Vec<bool>> = rows[..height].to_vec();
            rows.reverse();
            let bottom_rows: Vec<Vec<bool>> = rows[..height].to_vec();
            assert_eq!(top_rows.len(), bottom_rows.len());
            assert_eq!(top_rows.len(), height);

            for (y, (top, bottom)) in top_rows
                .into_iter()
                .zip(bottom_rows.into_iter())
                .enumerate()
            {
                for (x, (top, bottom)) in top.into_iter().zip(bottom.into_iter()).enumerate() {
                    new_rows[y][x] = top || bottom
                }
            }
            rows = new_rows
        }
    }
    (rows, height, width)
}
