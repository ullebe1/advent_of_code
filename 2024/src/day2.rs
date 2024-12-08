use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
pub struct Input {
    reports: Vec<Vec<usize>>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Input {
    let lines = input.lines();

    let reports = lines.map(|line| {
        line.split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect()
    }).collect::<Vec<Vec<usize>>>();

    Input { reports }
}

#[aoc(day2, part1)]
pub fn part1(input: &Input) -> usize {
    let mut reports = input.reports.clone();

    let mut safe_reports = 0;

    for report in reports.iter_mut() {
        let mut previous = None;
        let mut direction: Option<Direction> = None;
        let mut unsafe_report = false;

        for level in report.iter() {
            let (unsafe_report_current, new_direction) = is_a_safe_transition(previous, *level, direction);

            if unsafe_report_current {
                unsafe_report = true;
                break;
            }
            
            direction = new_direction;
            previous = Some(*level);
        }

        if !unsafe_report {
            safe_reports += 1;
        }
    }

    safe_reports
}

#[aoc(day2, part2)]
pub fn part2(input: &Input) -> usize {
    let mut reports = input.reports.clone();

    let mut safe_reports = 0;

    for report in reports.iter_mut() {
        let mut previous = None;
        let mut direction: Option<Direction> = None;
        let mut unsafe_report = false;

        for level in report.iter() {
            let (unsafe_report_current, new_direction) = is_a_safe_transition(previous, *level, direction);

            if unsafe_report_current {
                unsafe_report = true;
                break;
            }
            
            direction = new_direction;
            previous = Some(*level);
        }

        if unsafe_report {
            let mut counter = 0;
            let mut temp_unsafe_report = false;
            
            while counter < report.len() {
                previous = None;
                direction = None;
                let mut report = report.clone();
                report.remove(counter);
                for level in report.iter() {
                    let (unsafe_report_current, new_direction) = is_a_safe_transition(previous, *level, direction);
                    
                    if unsafe_report_current {
                        temp_unsafe_report = true;
                        break;
                    }
                    

                    direction = new_direction;
                    previous = Some(*level);
                    temp_unsafe_report = false;
                }
                counter += 1;

                if !temp_unsafe_report {
                    unsafe_report = false;
                    break;
                }
            }
        }

        if !unsafe_report {
            safe_reports += 1;
        }
    }

    safe_reports
}

fn is_a_safe_transition(previous: Option<usize>, level: usize, mut direction: Option<Direction>) -> (bool, Option<Direction>) {
    let mut unsafe_report = false;

    if previous.is_some() {
        if level.abs_diff(previous.unwrap()) > 3 || level.abs_diff(previous.unwrap()) == 0 {
            unsafe_report = true;
        }

        if direction.is_none() {
            if level > previous.unwrap() {
                direction = Some(Direction::Up);
            } else {
                direction = Some(Direction::Down);
            }
        } else {
            if direction.unwrap() == Direction::Up && level < previous.unwrap() {
                unsafe_report = true;
                direction = Some(Direction::Down);
            } else if direction.clone().unwrap() == Direction::Down && level > previous.unwrap() {
                unsafe_report = true;
                direction = Some(Direction::Up);
            }
        }
    }


    (unsafe_report, direction)
}
