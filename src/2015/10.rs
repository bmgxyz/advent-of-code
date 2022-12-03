use crate::util::AdventResult;

pub fn part_1(input: &str) -> AdventResult {
    let mut sequence = input.to_string();
    for _ in 0..40 {
        let mut new_sequence = Vec::new();
        let mut digit = sequence.chars().next().unwrap();
        let mut run = 0;
        for chr in sequence.chars() {
            if digit == chr {
                run += 1;
            } else {
                new_sequence.push(run.to_string());
                new_sequence.push(digit.to_string());
                digit = chr;
                run = 1;
            }
        }
        new_sequence.push(run.to_string());
        new_sequence.push(digit.to_string());
        sequence = new_sequence.iter().fold(String::new(), |acc, s| acc + s);
    }
    Ok(sequence.len() as u64)
}

#[cfg(test)]
use crate::util::check_solution;

#[test]
fn test_part_1() {
    check_solution("1", 82_350, &part_1);
}
