use crate::util::{parse_u64, AdventResult, AdventSolution};

pub fn part_1(input: &str) -> AdventResult {
    let values = parse_u64(input);
    for a in values.iter() {
        for b in values.iter() {
            if a + b == 2020 {
                return Ok(AdventSolution::from(a * b));
            }
        }
    }
    Err(String::from("Failed to find solution"))
}

pub fn part_2(input: &str) -> AdventResult {
    let values = parse_u64(input);
    for a in values.iter() {
        for b in values.iter() {
            for c in values.iter() {
                if a + b + c == 2020 {
                    return Ok(AdventSolution::from(a * b * c));
                }
            }
        }
    }
    Err(String::from("Failed to find solution"))
}

#[cfg(test)]
const DAY_01_SAMPLE_INPUT: &str = "1721\n\
    979\n\
    366\n\
    299\n\
    675\n\
    1456\n";

#[cfg(test)]
use crate::util::check_solution;

#[test]
fn test_part_1() {
    check_solution(&DAY_01_SAMPLE_INPUT.to_string(), 514579, &part_1);
}

#[test]
fn test_part_2() {
    check_solution(&DAY_01_SAMPLE_INPUT.to_string(), 241861950, &part_2);
}
