use crate::util::{AdventResult, AdventSolution};

pub fn part_1(input: &str) -> AdventResult {
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
    Ok(AdventSolution::from(gamma_rate * epsilon_rate))
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

pub fn part_2(input: &str) -> AdventResult {
    let remaining_numbers: Vec<&str> = input.lines().collect();
    let oxygen_rating = filter_numbers(&remaining_numbers, Gas::Oxygen);
    let carbon_dioxide_rating = filter_numbers(&remaining_numbers, Gas::CarbonDioxide);
    Ok(AdventSolution::from(
        u64::from_str_radix(&oxygen_rating, 2).unwrap()
            * u64::from_str_radix(&carbon_dioxide_rating, 2).unwrap(),
    ))
}

#[cfg(test)]
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

#[cfg(test)]
use crate::util::check_solution;

#[test]
fn test_part_1() {
    check_solution(&DAY_03_SAMPLE_INPUT.to_string(), 198, &part_1);
}

#[test]
fn test_part_2() {
    check_solution(&DAY_03_SAMPLE_INPUT.to_string(), 230, &part_2);
}
