use regex::Regex;
use std::collections::HashMap;

mod util;

pub fn solve_day_01_part_1(input: &str) -> Result<u64, String> {
    let depths = util::parse_u64(input);
    if depths.len() < 2 {
        return Err(format!(
            "Not enough depth measurements: found {}",
            depths.len()
        ));
    }
    let mut increase_count = 0;
    let mut last_depth = depths[0];
    for depth in depths[1..].iter() {
        if *depth > last_depth {
            increase_count += 1;
        }
        last_depth = *depth;
    }
    Ok(increase_count)
}

pub fn solve_day_01_part_2(input: &str) -> Result<u64, String> {
    let depths = util::parse_u64(input);
    if depths.len() < 4 {
        return Err(format!(
            "Not enough depth measurements: found {}",
            depths.len()
        ));
    }
    let mut increase_count = 0;
    let mut last_measurement = depths[0] + depths[1] + depths[2];
    let mut new_measurement: u64;
    for i in 3..depths.len() {
        new_measurement = last_measurement - depths[i - 3] + depths[i];
        if new_measurement > last_measurement {
            increase_count += 1;
        }
        last_measurement = new_measurement;
    }
    Ok(increase_count)
}

pub fn solve_day_02_part_1(input: &str) -> Result<u64, String> {
    let mut horizontal_position = 0;
    let mut depth = 0;
    for line in input.lines() {
        let (direction, amount) = {
            let s: Vec<&str> = line.split(' ').collect();
            (s[0], s[1].parse::<u64>().unwrap())
        };
        match direction {
            "forward" => horizontal_position += amount,
            "up" => depth -= amount,
            "down" => depth += amount,
            _ => unreachable!(),
        };
    }
    Ok(horizontal_position * depth)
}

pub fn solve_day_02_part_2(input: &str) -> Result<u64, String> {
    let mut aim = 0;
    let mut depth = 0;
    let mut horizontal_position = 0;
    for line in input.lines() {
        let (direction, amount) = {
            let s: Vec<&str> = line.split(' ').collect();
            (s[0], s[1].parse::<u64>().unwrap())
        };
        match direction {
            "forward" => {
                horizontal_position += amount;
                depth += aim * amount;
            }
            "up" => aim -= amount,
            "down" => aim += amount,
            _ => unreachable!(),
        };
    }
    Ok(horizontal_position * depth)
}

pub fn solve_day_03_part_1(input: &str) -> Result<u64, String> {
    let length = input.lines().next().unwrap().len();
    let mut bit_counts = vec![0; length];
    for line in input.lines() {
        for (idx, ch) in line.chars().enumerate() {
            if ch == '1' {
                bit_counts[idx] += 1;
            } else {
                bit_counts[idx] -= 1;
            }
        }
    }
    let most_common_bits: Vec<bool> = bit_counts.iter().map(|b| b > &0).collect();
    let (gamma_rate, epsilon_rate) = {
        let (mut gamma_rate, mut epsilon_rate) = (0, 0);
        for (idx, bit) in most_common_bits.iter().rev().enumerate() {
            if *bit {
                gamma_rate += u64::pow(2, idx as u32);
            } else {
                epsilon_rate += u64::pow(2, idx as u32);
            }
        }
        (gamma_rate, epsilon_rate)
    };
    Ok(gamma_rate * epsilon_rate)
}

enum Gas {
    Oxygen,
    CarbonDioxide,
}

fn filter_numbers(numbers: &[&str], gas: Gas) -> String {
    let mut remaining_numbers = numbers.to_owned();
    let length = remaining_numbers[0].len();
    let mut bit_pos = 0;
    let mut bit_count;
    let mut digit: char;
    while remaining_numbers.len() > 1 && bit_pos < length {
        bit_count = 0;
        for number in remaining_numbers.iter() {
            if number.chars().nth(bit_pos).unwrap() == '1' {
                bit_count += 1;
            } else {
                bit_count -= 1;
            }
        }
        let mut idx = 0;
        while idx < remaining_numbers.len() {
            digit = remaining_numbers[idx].chars().nth(bit_pos).unwrap();
            match gas {
                Gas::Oxygen => {
                    if bit_count >= 0 && digit == '0' || bit_count < 0 && digit == '1' {
                        remaining_numbers.remove(idx);
                    } else {
                        idx += 1;
                    }
                }
                Gas::CarbonDioxide => {
                    if bit_count >= 0 && digit == '1' || bit_count < 0 && digit == '0' {
                        remaining_numbers.remove(idx);
                    } else {
                        idx += 1;
                    }
                }
            }
        }
        bit_pos += 1;
    }
    remaining_numbers[0].to_string()
}

pub fn solve_day_03_part_2(input: &str) -> Result<u64, String> {
    let remaining_numbers: Vec<&str> = input.lines().collect();
    let oxygen_rating = filter_numbers(&remaining_numbers, Gas::Oxygen);
    let carbon_dioxide_rating = filter_numbers(&remaining_numbers, Gas::CarbonDioxide);
    Ok(u64::from_str_radix(&oxygen_rating, 2).unwrap()
        * u64::from_str_radix(&carbon_dioxide_rating, 2).unwrap())
}

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

pub fn solve_day_04_part_1(input: &str) -> Result<u64, String> {
    let (numbers, mut boards) = parse_day_04_input(input);
    for number in numbers.iter() {
        for board in boards.iter_mut() {
            apply_new_number(*number, board);
            if is_winner(board) {
                return Ok(compute_score(board, *number));
            }
        }
    }
    Err(String::from("Failed to find a winning board"))
}

pub fn solve_day_04_part_2(input: &str) -> Result<u64, String> {
    let (numbers, mut boards) = parse_day_04_input(input);
    let mut winning_boards = vec![false; boards.len()];
    for number in numbers.iter() {
        for (idx, board) in boards.iter_mut().enumerate() {
            apply_new_number(*number, board);
            if is_winner(board) && !winning_boards[idx] {
                winning_boards[idx] = true;
                if winning_boards.iter().filter(|b| !**b).count() == 0 {
                    return Ok(compute_score(board, *number));
                }
            }
        }
    }
    Err(String::from("Failed to find the last winning board"))
}

fn parse_day_05_input(input: &str) -> Vec<(u16, u16, u16, u16)> {
    let re = Regex::new(r"(?P<x1>\d+),(?P<y1>\d+) -> (?P<x2>\d+),(?P<y2>\d+)").unwrap();
    input
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            let (x1, y1, x2, y2) = (
                caps["x1"].parse::<u16>().unwrap(),
                caps["y1"].parse::<u16>().unwrap(),
                caps["x2"].parse::<u16>().unwrap(),
                caps["y2"].parse::<u16>().unwrap(),
            );
            (x1, y1, x2, y2)
        })
        .collect()
}

fn increment_visited(visited: &mut HashMap<(u16, u16), u32>, point: (u16, u16)) {
    let (x, y) = point;
    if let Some(p) = visited.get_mut(&(x, y)) {
        *p += 1;
    } else {
        visited.insert((x, y), 1);
    };
}

pub fn solve_day_05_part_1(input: &str) -> Result<u64, String> {
    let mut visited: HashMap<(u16, u16), u32> = HashMap::new();
    for (x1, y1, x2, y2) in parse_day_05_input(input) {
        if x1 == x2 {
            for y in u16::min(y1, y2)..=u16::max(y1, y2) {
                increment_visited(&mut visited, (x1, y));
            }
        } else if y1 == y2 {
            for x in u16::min(x1, x2)..=u16::max(x1, x2) {
                increment_visited(&mut visited, (x, y1));
            }
        }
    }
    Ok(visited.values().filter(|v| v >= &&2).count() as u64)
}

pub fn solve_day_05_part_2(input: &str) -> Result<u64, String> {
    let mut visited: HashMap<(u16, u16), u32> = HashMap::new();
    for (x1, y1, x2, y2) in parse_day_05_input(input) {
        if x1 == x2 {
            for y in u16::min(y1, y2)..=u16::max(y1, y2) {
                increment_visited(&mut visited, (x1, y));
            }
        } else if y1 == y2 {
            for x in u16::min(x1, x2)..=u16::max(x1, x2) {
                increment_visited(&mut visited, (x, y1));
            }
        } else {
            let (mut x, mut y) = (x1, y1);
            while x != x2 && y != y2 {
                increment_visited(&mut visited, (x, y));
                if x2 > x1 {
                    x += 1;
                } else {
                    x -= 1;
                }
                if y2 > y1 {
                    y += 1;
                } else {
                    y -= 1;
                }
            }
            increment_visited(&mut visited, (x, y));
        }
    }
    Ok(visited.values().filter(|v| v >= &&2).count() as u64)
}

fn simulate_lanternfish(input: &str, days: u16) -> u64 {
    let initial_lanternfish: Vec<u64> = input
        .split(',')
        .map(|f| f.trim_end().parse::<u64>().unwrap())
        .collect();
    let mut lanternfish = [0u64; 9];
    // count the fish in each timer state
    for fish in initial_lanternfish {
        lanternfish[fish as usize] += 1;
    }
    let mut zero_fish;
    for _ in 0..days {
        zero_fish = lanternfish[0];
        // subtract all fish timers by shifting values left
        for idx in 0..8 {
            lanternfish[idx] = lanternfish[idx + 1];
        }
        // all the fish that were at zero are now at six
        lanternfish[6] += zero_fish;
        // and they reproduce, which results in an equal number of fish at eight
        lanternfish[8] = zero_fish;
    }
    lanternfish.iter().sum::<u64>()
}

pub fn solve_day_06_part_1(input: &str) -> Result<u64, String> {
    Ok(simulate_lanternfish(input, 80))
}

pub fn solve_day_06_part_2(input: &str) -> Result<u64, String> {
    Ok(simulate_lanternfish(input, 256))
}

#[cfg(test)]
mod test {
    use super::*;
    use util::check_solution;

    const DAY_01_SAMPLE_INPUT: &str = "199\n\
        200\n\
        208\n\
        210\n\
        200\n\
        207\n\
        240\n\
        269\n\
        260\n\
        263\n";

    #[test]
    fn day_01_part_1() {
        check_solution(&DAY_01_SAMPLE_INPUT.to_string(), 7, &solve_day_01_part_1);
    }

    #[test]
    fn day_01_part_2() {
        check_solution(&DAY_01_SAMPLE_INPUT.to_string(), 5, &solve_day_01_part_2);
    }

    const DAY_02_SAMPLE_INPUT: &str = "forward 5\n\
        down 5\n\
        forward 8\n\
        up 3\n\
        down 8\n\
        forward 2\n";

    #[test]
    fn day_02_part_1() {
        check_solution(&DAY_02_SAMPLE_INPUT.to_string(), 150, &solve_day_02_part_1);
    }

    #[test]
    fn day_02_part_2() {
        check_solution(&DAY_02_SAMPLE_INPUT.to_string(), 900, &solve_day_02_part_2);
    }

    const DAY_03_SAMPLE_INPUT: &str = "00100\n\
        11110\n\
        10110\n\
        10111\n\
        10101\n\
        01111\n\
        00111\n\
        11100\n\
        10000\n\
        11001\n\
        00010\n\
        01010\n";

    #[test]
    fn day_03_part_1() {
        check_solution(&DAY_03_SAMPLE_INPUT.to_string(), 198, &solve_day_03_part_1);
    }

    #[test]
    fn day_03_part_2() {
        check_solution(&DAY_03_SAMPLE_INPUT.to_string(), 230, &solve_day_03_part_2);
    }

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

    #[test]
    fn day_04_part_1() {
        check_solution(&DAY_04_SAMPLE_INPUT.to_string(), 4512, &solve_day_04_part_1);
    }

    #[test]
    fn day_04_part_2() {
        check_solution(&DAY_04_SAMPLE_INPUT.to_string(), 1924, &solve_day_04_part_2);
    }

    const DAY_05_SAMPLE_INPUT: &str = "0,9 -> 5,9\n\
        8,0 -> 0,8\n\
        9,4 -> 3,4\n\
        2,2 -> 2,1\n\
        7,0 -> 7,4\n\
        6,4 -> 2,0\n\
        0,9 -> 2,9\n\
        3,4 -> 1,4\n\
        0,0 -> 8,8\n\
        5,5 -> 8,2\n";

    #[test]
    fn day_05_part_1() {
        check_solution(&DAY_05_SAMPLE_INPUT.to_string(), 5, &solve_day_05_part_1);
    }

    #[test]
    fn day_05_part_2() {
        check_solution(&DAY_05_SAMPLE_INPUT.to_string(), 12, &solve_day_05_part_2);
        // I found this extra test case helpful
        let second_test = "3,3 -> 4,4\n3,3 -> 4,4";
        check_solution(&second_test.to_string(), 2, &solve_day_05_part_2);
    }

    const DAY_06_SAMPLE_INPUT: &str = "3,4,3,1,2\n";

    #[test]
    fn day_06_part_1() {
        check_solution(&DAY_06_SAMPLE_INPUT, 5934, &solve_day_06_part_1);
    }

    #[test]
    fn day_06_part_2() {
        check_solution(&DAY_06_SAMPLE_INPUT, 26984457539, &solve_day_06_part_2);
        unimplemented!();
    }
}
