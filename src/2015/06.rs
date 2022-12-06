use regex::Regex;

use crate::util::{AdventResult, AdventSolution};

pub fn part_1(input: &str) -> AdventResult {
    let re = Regex::new(r"(turn on|toggle|turn off) (\d+),(\d+) through (\d+),(\d+)").unwrap();
    let mut lights = vec![vec![false; 1000]; 1000];
    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        let (action, x1, y1, x2, y2) = (
            &caps[1],
            caps[2].parse::<usize>().unwrap(),
            caps[3].parse::<usize>().unwrap(),
            caps[4].parse::<usize>().unwrap(),
            caps[5].parse::<usize>().unwrap(),
        );
        // I think it's more clear to explicitly index into lights rather than
        // construct an iterator over it.
        #[allow(clippy::needless_range_loop)]
        match action {
            "turn on" => {
                for row in y1..=y2 {
                    for col in x1..=x2 {
                        lights[row][col] = true;
                    }
                }
            }
            "toggle" => {
                for row in y1..=y2 {
                    for col in x1..=x2 {
                        lights[row][col] = !lights[row][col];
                    }
                }
            }
            "turn off" => {
                for row in y1..=y2 {
                    for col in x1..=x2 {
                        lights[row][col] = false;
                    }
                }
            }
            _ => unreachable!(),
        };
    }
    let mut enabled_lights = 0;
    for row in lights {
        for light in row {
            if light {
                enabled_lights += 1;
            }
        }
    }
    Ok(AdventSolution::from(enabled_lights))
}

pub fn part_2(input: &str) -> AdventResult {
    let re = Regex::new(r"(turn on|toggle|turn off) (\d+),(\d+) through (\d+),(\d+)").unwrap();
    let mut lights = vec![vec![0; 1000]; 1000];
    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        let (action, x1, y1, x2, y2) = (
            &caps[1],
            caps[2].parse::<usize>().unwrap(),
            caps[3].parse::<usize>().unwrap(),
            caps[4].parse::<usize>().unwrap(),
            caps[5].parse::<usize>().unwrap(),
        );
        // I think it's more clear to explicitly index into lights rather than
        // construct an iterator over it.
        #[allow(clippy::needless_range_loop)]
        match action {
            "turn on" => {
                for row in y1..=y2 {
                    for col in x1..=x2 {
                        lights[row][col] += 1;
                    }
                }
            }
            "toggle" => {
                for row in y1..=y2 {
                    for col in x1..=x2 {
                        lights[row][col] += 2;
                    }
                }
            }
            "turn off" => {
                for row in y1..=y2 {
                    for col in x1..=x2 {
                        if lights[row][col] > 0 {
                            lights[row][col] -= 1;
                        };
                    }
                }
            }
            _ => unreachable!(),
        };
    }
    let mut total_brightness = 0;
    for row in lights {
        for brightness in row {
            total_brightness += brightness
        }
    }
    Ok(AdventSolution::from(total_brightness))
}

#[cfg(test)]
use crate::util::check_solution;

#[test]
fn test_part_1() {
    check_solution("turn on 0,0 through 999,999\n", 1_000_000, &part_1);
    check_solution(
        "turn on 0,0 through 999,999\ntoggle 0,0 through 999,0",
        999_000,
        &part_1,
    );
    check_solution(
        "turn on 0,0 through 999,999\nturn off 499,499 through 500,500",
        999_996,
        &part_1,
    );
}

#[test]
fn test_part_2() {
    check_solution("turn on 0,0 through 0,0", 1, &part_2);
    check_solution("toggle 0,0 through 999,999", 2_000_000, &part_2);
}
