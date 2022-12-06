use crate::util::{AdventResult, AdventSolution};

pub fn part_1(input: &str) -> AdventResult {
    let mut floor = 0;
    for ch in input.chars() {
        if ch == '(' {
            floor += 1;
        } else if ch == ')' {
            floor -= 1;
        }
    }
    Ok(AdventSolution::from(floor))
}

pub fn part_2(input: &str) -> AdventResult {
    let mut floor = 0;
    for (idx, ch) in input.chars().enumerate() {
        if ch == '(' {
            floor += 1;
        } else if ch == ')' {
            floor -= 1;
        }
        if floor < 0 {
            return Ok(AdventSolution::from(idx + 1));
        }
    }
    Err(String::from("Failed to find the basement"))
}

#[cfg(test)]
use crate::util::check_solution;

#[test]
fn test_part_1() {
    check_solution("(())\n", 0, &part_1);
    check_solution("()()\n", 0, &part_1);
    check_solution("(((\n", 3, &part_1);
    check_solution("(()(()(\n", 3, &part_1);
    check_solution("))(((((\n", 3, &part_1);
}

#[test]
fn test_part_2() {
    check_solution(")\n", 1, &part_2);
    check_solution("()())\n", 5, &part_2);
}
