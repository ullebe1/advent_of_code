use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
pub struct Input {
    word_search: Vec<Vec<char>>,
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Input {
    let lines = input.lines();

    let word_search = lines.map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    Input { word_search }
}

#[aoc(day4, part1)]
pub fn part1(input: &Input) -> usize {
    let width = input.word_search[0].len();
    let height = input.word_search.len();

    let mut found_xmas = 0;

    for y in 0..height {
        for x in 0..width {
            if input.word_search[y][x] == 'X' {
                // Check horisontally to the right
                if x + 3 < width && input.word_search[y][x + 1] == 'M' && input.word_search[y][x + 2] == 'A' && input.word_search[y][x + 3] == 'S' {
                    found_xmas += 1;
                }
                // Check horisontally to the left
                if x >= 3 && input.word_search[y][x - 1] == 'M' && input.word_search[y][x - 2] == 'A' && input.word_search[y][x - 3] == 'S' {
                    found_xmas += 1;
                }
                // Check vertically down
                if y + 3 < height && input.word_search[y + 1][x] == 'M' && input.word_search[y + 2][x] == 'A' && input.word_search[y + 3][x] == 'S' {
                    found_xmas += 1;
                }
                // Check vertically up
                if y >= 3 && input.word_search[y - 1][x] == 'M' && input.word_search[y - 2][x] == 'A' && input.word_search[y - 3][x] == 'S' {
                    found_xmas += 1;
                }
                // Check diagonally down right
                if x + 3 < width && y + 3 < height && input.word_search[y + 1][x + 1] == 'M' && input.word_search[y + 2][x + 2] == 'A' && input.word_search[y + 3][x + 3] == 'S' {
                    found_xmas += 1;
                }
                // Check diagonally down left
                if x >= 3 && y + 3 < height && input.word_search[y + 1][x - 1] == 'M' && input.word_search[y + 2][x - 2] == 'A' && input.word_search[y + 3][x - 3] == 'S' {
                    found_xmas += 1;
                }
                // Check diagonally up right
                if x + 3 < width && y >= 3 && input.word_search[y - 1][x + 1] == 'M' && input.word_search[y - 2][x + 2] == 'A' && input.word_search[y - 3][x + 3] == 'S' {
                    found_xmas += 1;
                }
                // Check diagonally up left
                if x >= 3 && y >= 3 && input.word_search[y - 1][x - 1] == 'M' && input.word_search[y - 2][x - 2] == 'A' && input.word_search[y - 3][x - 3] == 'S' {
                    found_xmas += 1;
                }
            }
        }
    }

    found_xmas
}

#[aoc(day4, part2)]
pub fn part2(input: &Input) -> usize {
    let width = input.word_search[0].len();
    let height = input.word_search.len();

    let mut found_x_mas = 0;

    for y in 0..height {
        for x in 0..width {
            if input.word_search[y][x] == 'A' {
                if x >= 1 && y >= 1 && input.word_search[y - 1][x - 1] == 'M' {
                    if x + 1 < width && y + 1 < height && input.word_search[y + 1][x + 1] == 'S' {
                        if input.word_search[y - 1][x + 1] == 'M' {
                            if input.word_search[y + 1][x - 1] == 'S' {
                                found_x_mas += 1;
                            }
                        } else if input.word_search[y - 1][x + 1] == 'S' {
                            if input.word_search[y + 1][x - 1] == 'M' {
                                found_x_mas += 1;
                            }
                        }
                    }
                }
                if x >= 1 && y >= 1 && input.word_search[y - 1][x - 1] == 'S' {
                    if x + 1 < width && y + 1 < height && input.word_search[y + 1][x + 1] == 'M' {
                        if input.word_search[y - 1][x + 1] == 'M' {
                            if input.word_search[y + 1][x - 1] == 'S' {
                                found_x_mas += 1;
                            }
                        } else if input.word_search[y - 1][x + 1] == 'S' {
                            if input.word_search[y + 1][x - 1] == 'M' {
                                found_x_mas += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    found_x_mas
}