use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
        .as_slice()
        .windows(2)
        .fold(
            0,
            |sum, window| {
                if window[0] < window[1] {
                    sum + 1
                } else {
                    sum
                }
            },
        )
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
        .as_slice()
        .windows(3)
        .map::<usize, _>(|window| window.iter().sum())
        .collect::<Vec<usize>>()
        .as_slice()
        .windows(2)
        .fold(
            0,
            |sum, window| {
                if window[0] < window[1] {
                    sum + 1
                } else {
                    sum
                }
            },
        )
}
