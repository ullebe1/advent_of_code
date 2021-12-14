use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Copy, Debug)]
pub struct Octopus {
    energy: usize,
}

pub struct Input {
    rows: Vec<Vec<Octopus>>,
    width: usize,
    height: usize,
}

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Input {
    let lines = input.lines();

    let rows: Vec<Vec<Octopus>> = lines
        .map(|x| {
            x.chars()
                .map(|y| Octopus {
                    energy: String::from(y).parse::<usize>().unwrap(),
                })
                .collect()
        })
        .collect();

    Input {
        width: rows[0].len(),
        height: rows.len(),
        rows: rows,
    }
}

#[aoc(day11, part1)]
pub fn part1(input: &Input) -> usize {
    let mut octopi = input.rows.clone();
    let mut flashes = 0;
    for round in 0..100 {
        println!("After step {}:", round);
        for y in 0..input.height {
            for x in 0..input.width {
                (flashes, octopi) =
                    process_octopus(octopi, y, x, flashes, input.height, input.width);
            }
        }
        for y in 0..input.height {
            for x in 0..input.width {
                if octopi[y][x].energy > 9 {
                    octopi[y][x].energy = 0;
                }
                print!("{}", octopi[y][x].energy);
            }
            print!("\n");
        }
        println!("");
    }
    flashes
}

#[aoc(day11, part2)]
pub fn part2(input: &Input) -> usize {
    let mut octopi = input.rows.clone();
    let mut flashes = 0;
    let mut round = 0;
    loop {
        round = round + 1;
        println!("After step {}:", round);
        for y in 0..input.height {
            for x in 0..input.width {
                (flashes, octopi) =
                    process_octopus(octopi, y, x, flashes, input.height, input.width);
            }
        }
        for y in 0..input.height {
            for x in 0..input.width {
                if octopi[y][x].energy > 9 {
                    octopi[y][x].energy = 0;
                }
                print!("{}", octopi[y][x].energy);
            }
            print!("\n");
        }
        println!("");
        if octopi
            .clone()
            .into_iter()
            .flat_map(|x| x)
            .fold(0, |sum, x| sum + x.energy)
            == 0
        {
            break;
        }
    }
    round
}

pub fn process_octopus(
    mut octopi: Vec<Vec<Octopus>>,
    y: usize,
    x: usize,
    mut flashes: usize,
    height: usize,
    width: usize,
) -> (usize, Vec<Vec<Octopus>>) {
    octopi[y][x].energy = octopi[y][x].energy + 1;
    if octopi[y][x].energy == 10 {
        flashes = flashes + 1;
        for y in (y as isize) - 1..=(y as isize) + 1 {
            for x in (x as isize) - 1..=(x as isize) + 1 {
                if 0 <= y && y < height as isize && 0 <= x && x < width as isize {
                    (flashes, octopi) =
                        process_octopus(octopi, y as usize, x as usize, flashes, height, width);
                }
            }
        }
    }

    (flashes, octopi)
}
