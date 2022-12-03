use std::collections::HashMap;

use regex::Regex;

use crate::util::AdventResult;

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

pub fn part_1(input: &str) -> AdventResult {
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

pub fn part_2(input: &str) -> AdventResult {
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

#[cfg(test)]
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

#[cfg(test)]
use crate::util::check_solution;

#[test]
fn test_part_1() {
    check_solution(&DAY_05_SAMPLE_INPUT.to_string(), 5, &part_1);
}

#[test]
fn test_part_2() {
    check_solution(&DAY_05_SAMPLE_INPUT.to_string(), 12, &part_2);
    // I found this extra test case helpful
    let second_test = "3,3 -> 4,4\n3,3 -> 4,4";
    check_solution(&second_test.to_string(), 2, &part_2);
}
