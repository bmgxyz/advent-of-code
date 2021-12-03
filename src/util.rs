#![allow(dead_code)]

pub fn not_solved_yet(year: u16, day: u8, part: u8) -> Result<u64, String> {
    Err(format!(
        "{} day {} part {} isn't solved yet",
        year, day, part
    ))
}

pub fn check_solution(input: &str, output: u64, solution: &dyn Fn(&str) -> Result<u64, String>) {
    let solution_result = solution(input);
    assert!(solution_result.is_ok());
    assert_eq!(solution_result.unwrap(), output);
}

pub fn parse_u64(input: &str) -> Vec<u64> {
    input
        .split_terminator('\n')
        .map(|x| x.parse::<u64>().unwrap())
        .collect()
}
