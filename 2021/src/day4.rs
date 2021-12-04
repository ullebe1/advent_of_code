use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone)]
pub struct Row {
    numbers: Vec<Option<usize>>,
}

#[derive(Debug, Clone)]
pub struct Board {
    numbers: Vec<Row>,
}

#[derive(Debug)]
pub struct Input {
    numbers: Vec<usize>,
    boards: Vec<Board>,
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Input {
    let mut lines = input.lines();

    // Extract drawn Bingo numbers
    let numbers = lines.next().unwrap();
    let numbers = numbers
        .split(",")
        .into_iter()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    // Extract Bingo boards
    let mut boards = Vec::new();
    'boards: loop {
        let mut numbers = Vec::new();
        'numbers: loop {
            let line = lines.next();
            if let Some(l) = line {
                if let "" = l {
                    break 'numbers;
                } else {
                    numbers.push(Row {
                        numbers: l
                            .replace("  ", " ")
                            .split(" ")
                            .into_iter()
                            .filter(|x| x != &"") // remove items created by line starting with whitespace (why?!?)
                            .map(|x| Some(x.parse::<usize>().unwrap()))
                            .collect(),
                    })
                }
            } else {
                break 'boards;
            }
        }
        if numbers.len() != 0 {
            boards.push(Board { numbers: numbers })
        }
    }

    Input {
        numbers: numbers,
        boards: boards,
    }
}

#[aoc(day4, part1)]
pub fn part1(input: &Input) -> usize {
    let mut latest_number = 1;
    let mut boards = input.boards.clone();

    let mut winning_board: Option<Board> = None;
    'outer: for number in input.numbers.clone().into_iter() {
        latest_number = number;
        for board_number in 0..boards.len() {
            boards[board_number].remove_number(number);
            if boards[board_number].has_won() {
                winning_board = Some(boards[board_number].clone());
                break 'outer;
            }
        }
    }
    if let Some(board) = winning_board {
        return board.get_score(latest_number);
    } else {
        return 0;
    }
}

#[aoc(day4, part2)]
pub fn part2(input: &Input) -> usize {
    let mut latest_number = 1;
    let mut boards = input.boards.clone();

    let mut losing_board: Option<Board> = None;
    'outer: for number in input.numbers.clone().into_iter() {
        latest_number = number;
        let mut boards_winning_this_round: Vec<usize> = Vec::new();
        for board_number in 0..boards.len() {
            boards[board_number].remove_number(number);
            if boards[board_number].has_won() {
                if boards.len() == 1 {
                    losing_board = Some(boards[0].clone());
                    break 'outer;
                }
                boards_winning_this_round.push(board_number);
            }
        }
        //println!("Removed boards: {}", boards_winning_this_round.len());
        let mut removed_so_far_index = 0;
        for board in boards_winning_this_round {
            boards.remove(board - removed_so_far_index);
            removed_so_far_index = removed_so_far_index + 1;
        }
    }
    if let Some(board) = losing_board {
        return board.get_score(latest_number);
    } else {
        return 0;
    }
}

impl Board {
    fn remove_number(&mut self, number: usize) {
        self.numbers = self
            .numbers
            .clone()
            .into_iter()
            .map(|x| Row {
                numbers: {
                    x.numbers
                        .into_iter()
                        .map(|x| match x {
                            Some(y) => {
                                if y == number {
                                    None
                                } else {
                                    Some(y)
                                }
                            }
                            None => None,
                        })
                        .collect()
                },
            })
            .collect();
    }

    fn has_won(&self) -> bool {
        has_board_won(self)
    }

    fn get_score(&self, final_number: usize) -> usize {
        self.numbers.clone().into_iter().fold(0, |sum, x| {
            sum + x.numbers.into_iter().fold(0, |sum2, y| {
                sum2 + match y {
                    Some(z) => z,
                    None => 0,
                }
            })
        }) * final_number
    }
}

fn has_board_won(board: &Board) -> bool {
    let temp_board = Board {
        numbers: [
            board.numbers.clone(),
            board
                .clone()
                .numbers
                .into_iter()
                .enumerate()
                .map(|(index, _x)| get_column(board.numbers.clone(), index))
                .into_iter()
                .map(|x| Row { numbers: x })
                .collect::<Vec<Row>>(),
        ]
        .concat(),
    };
    temp_board.numbers.into_iter().any(|x| {
        x.numbers
            .into_iter()
            .filter(|y| y.is_some())
            .collect::<Vec<Option<usize>>>()
            .len()
            == 0
    })
}

fn get_column(matrix: Vec<Row>, index: usize) -> Vec<Option<usize>> {
    get_column_generic(
        matrix
            .into_iter()
            .map(|x| x.numbers)
            .collect::<Vec<Vec<Option<usize>>>>(),
        index,
    )
}

fn get_column_generic<T: std::marker::Copy>(matrix: Vec<Vec<T>>, index: usize) -> Vec<T> {
    matrix.into_iter().map(|x| x[index]).collect()
}
