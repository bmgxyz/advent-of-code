use regex::Regex;

use crate::util::AdventResult;

pub fn part_1(input: &str) -> AdventResult {
    let re = Regex::new(r"row (\d+), column (\d+)").unwrap();
    let caps = re.captures(input).unwrap();
    let (row, column) = (
        caps[1].parse::<i64>().unwrap(),
        caps[2].parse::<i64>().unwrap(),
    );
    let mut num_iterations = ((row + column - 2).pow(2) + (row + column - 2) + 2) / 2 + column - 2;
    // The pattern repeats, so skip as many complete cycles as we can.
    while num_iterations > 16_777_196 {
        num_iterations -= 16_777_196;
    }

    let mut code = 20_151_125;
    let m = 252_533;
    let n = 33_554_393;
    for _ in 0..num_iterations {
        code = (code * m) % n;
    }

    Ok(code)
}

#[cfg(test)]
use crate::util::check_solution;

#[test]
fn test_part_1() {
    check_solution("row 6, column 6", 27995004, &part_1);
}
