use crate::util::{AdventResult, AdventSolution};

type Board = Vec<Vec<(u8, bool)>>;

fn apply_new_number(number: u8, board: &mut Board) {
    #![allow(clippy::needless_range_loop)]
    for row in 0..5 {
        for col in 0..5 {
            if board[row][col].0 == number {
                board[row][col].1 = true;
            }
        }
    }
}

fn is_winner(board: &Board) -> bool {
    #![allow(clippy::ptr_arg)]
    // check rows
    for board_row in board.iter().take(5) {
        if board_row[0].1 && board_row[1].1 && board_row[2].1 && board_row[3].1 && board_row[4].1 {
            return true;
        }
    }
    // check columns
    for col in 0..5 {
        if board[0][col].1
            && board[1][col].1
            && board[2][col].1
            && board[3][col].1
            && board[4][col].1
        {
            return true;
        }
    }
    false
}

fn compute_score(board: &Board, last_number: u8) -> u64 {
    #![allow(clippy::needless_range_loop, clippy::ptr_arg)]
    let mut sum: u64 = 0;
    for row in 0..5 {
        for col in 0..5 {
            if !board[row][col].1 {
                sum += board[row][col].0 as u64;
            }
        }
    }
    sum * (last_number as u64)
}

fn parse_day_04_input(input: &str) -> (Vec<u8>, Vec<Board>) {
    let mut lines = input.lines().peekable();
    let numbers: Vec<u8> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse::<u8>().unwrap())
        .collect();
    lines.next();
    let mut boards: Vec<Board> = Vec::new();
    let mut new_board: Board;
    while lines.peek().is_some() {
        new_board = Vec::new();
        for _row in 1..=5 {
            new_board.push(
                lines
                    .next()
                    .unwrap()
                    .split_ascii_whitespace()
                    .map(|n| (n.parse::<u8>().unwrap(), false))
                    .collect(),
            );
        }
        boards.push(new_board);
        lines.next();
    }
    (numbers, boards)
}

pub fn part_1(input: &str) -> AdventResult {
    let (numbers, mut boards) = parse_day_04_input(input);
    for number in numbers.iter() {
        for board in boards.iter_mut() {
            apply_new_number(*number, board);
            if is_winner(board) {
                return Ok(AdventSolution::from(compute_score(board, *number)));
            }
        }
    }
    Err(String::from("Failed to find a winning board"))
}

pub fn part_2(input: &str) -> AdventResult {
    let (numbers, mut boards) = parse_day_04_input(input);
    let mut winning_boards = vec![false; boards.len()];
    for number in numbers.iter() {
        for (idx, board) in boards.iter_mut().enumerate() {
            apply_new_number(*number, board);
            if is_winner(board) && !winning_boards[idx] {
                winning_boards[idx] = true;
                if winning_boards.iter().filter(|b| !**b).count() == 0 {
                    return Ok(AdventSolution::from(compute_score(board, *number)));
                }
            }
        }
    }
    Err(String::from("Failed to find the last winning board"))
}

#[cfg(test)]
const DAY_04_SAMPLE_INPUT: &str =
    "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\n\
    \n\
    22 13 17 11  0\n\
    8  2 23  4 24\n\
    21  9 14 16  7\n\
    6 10  3 18  5\n\
    1 12 20 15 19\n\
    \n\
    3 15  0  2 22\n\
    9 18 13 17  5\n\
    19  8  7 25 23\n\
    20 11 10 24  4\n\
    14 21 16 12  6\n\
    \n\
    14 21 17 24  4\n\
    10 16 15  9 19\n\
    18  8 23 26 20\n\
    22 11 13  6  5\n\
    2  0 12  3  7";

#[cfg(test)]
use crate::util::check_solution;

#[test]
fn test_part_1() {
    check_solution(&DAY_04_SAMPLE_INPUT.to_string(), 4512, &part_1);
}

#[test]
fn test_part_2() {
    check_solution(&DAY_04_SAMPLE_INPUT.to_string(), 1924, &part_2);
}
