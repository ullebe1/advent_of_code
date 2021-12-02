use aoc_runner_derive::{aoc};

#[aoc(day2, part1)]
pub fn part1(input: &str) -> usize {
    let mut depth = 0;
    let mut distance = 0;
    
    let lines = input.lines();
    for line in lines {
        let mut parts = line.split(" ");
        match parts.next().unwrap() {
            "forward" => distance = distance + parts.next().unwrap().parse::<usize>().unwrap(),
            "down" => depth = depth + parts.next().unwrap().parse::<usize>().unwrap(),
            "up" => depth = depth - parts.next().unwrap().parse::<usize>().unwrap(),
            _ => unreachable!()
        }
    }

    return depth * distance;
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> usize {
    let mut depth = 0;
    let mut distance = 0;
    let mut aim = 0;
    
    let lines = input.lines();
    for line in lines {
        let mut parts = line.split(" ");
        match parts.next().unwrap() {
            "forward" => {
                let x = parts.next().unwrap().parse::<usize>().unwrap();
                distance = distance + x;
                depth = depth + (aim * x);
            },
            "down" => {
                aim = aim + parts.next().unwrap().parse::<usize>().unwrap()
            },
            "up" => {
                aim = aim - parts.next().unwrap().parse::<usize>().unwrap()
            },
            _ => unreachable!()
        }
    }

    return depth * distance;
}