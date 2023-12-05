use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
pub struct Input {
    calibrations: Vec<String>,
}

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Input {
    let lines = input.lines();

    let calibrations = lines.map(|line| line.to_string()).collect::<Vec<String>>();

    Input { calibrations }
}

#[aoc(day1, part1)]
pub fn part1(input: &Input) -> usize {
    let mut digits = Vec::new();

    for calibration in input.calibrations.clone() {
        let mut first = None;
        let mut second = None;
        println!("{}", calibration);
        for c in calibration.chars() {
            println!("  {}", c);
            if c.is_digit(10) {
                if first.is_none() {
                    first = Some(c);
                    println!("    first: {}", first.unwrap());
                } else {
                    second = Some(c);
                    println!("    second: {}", second.unwrap());
                }
            }
        }
        if first.is_some() && second.is_some() {
            digits.push(first.unwrap().to_string() + &second.unwrap().to_string());
        } else if first.is_some() {
            digits.push(first.unwrap().to_string() + &first.unwrap().to_string());
        }
    }

    digits.iter().map(|s| s.parse::<usize>().unwrap()).sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &Input) -> usize {
    let mut digits = Vec::new();

    for mut calibration in input.calibrations.clone() {
        let mut first = None;
        let mut second = None;
        println!("{}", calibration);
        loop {
            if calibration.is_empty() {
                break;
            }
            let digit = match calibration.chars().collect::<Vec<char>>().as_slice() {
                ['1', ..] => Some('1'),
                ['2', ..] => Some('2'),
                ['3', ..] => Some('3'),
                ['4', ..] => Some('4'),
                ['5', ..] => Some('5'),
                ['6', ..] => Some('6'),
                ['7', ..] => Some('7'),
                ['8', ..] => Some('8'),
                ['9', ..] => Some('9'),
                ['0', ..] => Some('0'),
                ['o', 'n', 'e', ..] => Some('1'),
                ['t', 'w', 'o', ..] => Some('2'),
                ['t', 'h', 'r', 'e', 'e', ..] => Some('3'),
                ['f', 'o', 'u', 'r', ..] => Some('4'),
                ['f', 'i', 'v', 'e', ..] => Some('5'),
                ['s', 'i', 'x', ..] => Some('6'),
                ['s', 'e', 'v', 'e', 'n', ..] => Some('7'),
                ['e', 'i', 'g', 'h', 't', ..] => Some('8'),
                ['n', 'i', 'n', 'e', ..] => Some('9'),
                ['z', 'e', 'r', 'o', ..] => Some('0'),
                _ => None,
            };

            calibration = calibration.chars().skip(1).collect::<String>();

            if digit.is_some() {
                if first.is_none() {
                    first = digit;
                    println!("    first: {}", first.unwrap());
                } else {
                    second = digit;
                    println!("    second: {}", second.unwrap());
                }
            }
        }

        if first.is_some() && second.is_some() {
            digits.push(first.unwrap().to_string() + &second.unwrap().to_string());
        } else if first.is_some() {
            digits.push(first.unwrap().to_string() + &first.unwrap().to_string());
        }
    }

    digits.iter().map(|s| s.parse::<usize>().unwrap()).sum()
}
