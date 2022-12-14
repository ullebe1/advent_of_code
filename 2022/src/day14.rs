use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone)]
pub enum Content {
    Sand,
    Rock,
}

#[derive(Debug, Clone)]
pub struct Input {
    grid: Vec<Vec<Option<Content>>>,
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Input {
    let lines = input.lines().collect::<Vec<&str>>();
    let lines = lines
        .into_iter()
        .map(|line| line.split(" -> ").collect::<Vec<_>>())
        .map(|line_pairs| {
            line_pairs
                .into_iter()
                .map(|pair| {
                    let pair = pair
                        .split(',')
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect::<Vec<_>>();
                    assert_eq!(pair.len(), 2);
                    (pair[0], pair[1])
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let limits = find_limits(&lines);
    println!("{:#?}", limits);
    let mut grid = vec![vec![None; limits.0 + 1]; limits.1 + 1];

    // Insert rocks
    for line in lines {
        for window in line.windows(2) {
            assert_eq!(window.len(), 2);
            // if horisontal
            if window[0].1 == window[1].1 {
                println!(
                    "Hor {} {} {} {}",
                    window[0].0, window[1].0, window[0].1, window[1].1
                );
                for i in window[1].0..=window[0].0 {
                    println!("{}", i);
                    let item = &mut grid[window[0].1][i];
                    *item = Some(Content::Rock);
                }

            // else vertical
            } else {
                println!(
                    "Ver {} {} {} {}",
                    window[0].0, window[1].0, window[0].1, window[1].1
                );
                for i in window[0].1..=window[1].1 {
                    let item = &mut grid[i][window[0].0];
                    *item = Some(Content::Rock);
                }
            }
        }
    }

    let pretty_grid = (&grid)
        .into_iter()
        .map(|x| {
            x.into_iter()
                .map(|y| match y {
                    Some(_) => 1,
                    None => 0,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    println!("{:?}", pretty_grid);

    Input { grid: grid }
}

#[aoc(day14, part1)]
pub fn part1(input: &Input) -> String {
    unimplemented!()
}

#[aoc(day14, part2)]
pub fn part2(input: &Input) -> String {
    unimplemented!()
}

pub fn find_limits(pairs: &Vec<Vec<(usize, usize)>>) -> (usize, usize) {
    let pairs = pairs.into_iter().flatten().collect::<Vec<_>>();

    let max_x = (&pairs).into_iter().map(|x| x.0).max().unwrap();
    let max_y = (&pairs).into_iter().map(|x| x.1).max().unwrap();

    (max_x, max_y)
}
