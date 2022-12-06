use crate::util::{AdventResult, AdventSolution};

pub fn part_1(input: &str) -> AdventResult {
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
    Ok(AdventSolution::from(horizontal_position * depth))
}

pub fn part_2(input: &str) -> AdventResult {
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
    Ok(AdventSolution::from(horizontal_position * depth))
}

#[cfg(test)]
const DAY_02_SAMPLE_INPUT: &str = "forward 5\n\
        down 5\n\
        forward 8\n\
        up 3\n\
        down 8\n\
        forward 2\n";

#[cfg(test)]
use crate::util::check_solution;

#[test]
fn test_part_1() {
    check_solution(&DAY_02_SAMPLE_INPUT.to_string(), 150, &part_1);
}

#[test]
fn test_part_2() {
    check_solution(&DAY_02_SAMPLE_INPUT.to_string(), 900, &part_2);
}
