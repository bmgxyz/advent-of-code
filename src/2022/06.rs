use std::collections::VecDeque;

use crate::util::{AdventResult, AdventSolution};

fn find_start_packet(input: &str, length: usize) -> AdventResult {
    let mut is_start_packet;
    let mut buffer = VecDeque::from(vec![' '; length]);
    for (idx, c) in input.chars().enumerate() {
        is_start_packet = true;
        buffer.push_back(c);
        buffer.pop_front();
        for a in buffer.iter() {
            if buffer.contains(&' ') || buffer.iter().filter(|x| *x == a).count() > 1 {
                is_start_packet = false;
                break;
            }
        }
        if is_start_packet {
            return Ok(AdventSolution::from(idx + 1));
        }
    }
    Err("Failed to find start packet".to_string())
}

pub fn part_1(input: &str) -> AdventResult {
    find_start_packet(input, 4)
}

pub fn part_2(input: &str) -> AdventResult {
    find_start_packet(input, 14)
}

#[cfg(test)]
use crate::util::check_solution;

#[test]
fn test_part_1() {
    check_solution("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7, &part_1);
    check_solution("bvwbjplbgvbhsrlpgdmjqwftvncz", 5, &part_1);
    check_solution("nppdvjthqldpwncqszvftbrmjlhg", 6, &part_1);
    check_solution("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10, &part_1);
    check_solution("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11, &part_1);
}

#[test]
fn test_part_2() {
    check_solution("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19, &part_2);
    check_solution("bvwbjplbgvbhsrlpgdmjqwftvncz", 23, &part_2);
    check_solution("nppdvjthqldpwncqszvftbrmjlhg", 23, &part_2);
    check_solution("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29, &part_2);
    check_solution("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26, &part_2);
}
