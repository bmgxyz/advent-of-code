use crate::util::{AdventResult, AdventSolution};

fn compute_item_priority(item: char) -> Result<u64, String> {
    if item.is_ascii_alphabetic() {
        match item {
            // Lowercase letters range in priority from 1 to 26, inclusive.
            c if c.is_ascii_lowercase() => Ok((c as u8 - 0x60) as u64),
            // Uppercase letters range in priority from 27 to 52, inclusive.
            c if c.is_ascii_uppercase() => Ok((c as u8 - 0x40 + 26) as u64),
            _ => unreachable!(),
        }
    } else {
        Err(format!("Found invalid rucksack item: '{}'", item))
    }
}

pub fn part_1(input: &str) -> AdventResult {
    let mut total_priority = 0;
    for rucksack in input.lines() {
        if rucksack.len() % 2 != 0 {
            return Err(format!("Rucksack is not balanced: '{}'", rucksack));
        }
        if rucksack.is_empty() {
            continue;
        }
        let first_compartment = rucksack
            .chars()
            .take(rucksack.len() / 2)
            .collect::<String>();
        let second_compartment = rucksack
            .chars()
            .skip(rucksack.len() / 2)
            .collect::<String>();
        let mut found_a_match = false;
        for item in first_compartment.chars() {
            if second_compartment.contains(item) {
                total_priority += compute_item_priority(item)?;
                found_a_match = true;
                break;
            }
        }
        if !found_a_match {
            return Err(format!(
                "Failed to find a match in rucksack: '{}'",
                rucksack
            ));
        }
    }
    Ok(AdventSolution::from(total_priority))
}

pub fn part_2(input: &str) -> AdventResult {
    let mut total_priority = 0;
    // Subtract one here to account for the trailing new line.
    if (input.lines().count() - 1) % 3 != 0 {
        return Err("Elves are not in groups of three".to_string());
    }
    let mut rucksack_iter = input.lines();
    loop {
        let first_rucksack = match rucksack_iter.next() {
            // If there's another line and it's not blank, then we have at least
            // one more group to process.
            Some(r) if !r.is_empty() => r,
            // We've processed all of the groups, so we're done.
            _ => break,
        };
        let second_rucksack = rucksack_iter.next().unwrap();
        let third_rucksack = rucksack_iter.next().unwrap();
        let mut found_a_match = false;
        for item in first_rucksack.chars() {
            if second_rucksack.contains(item) && third_rucksack.contains(item) {
                total_priority += compute_item_priority(item)?;
                found_a_match = true;
                break;
            }
        }
        if !found_a_match {
            return Err(format!(
                "Failed to find a match in rucksack group: '{}', '{}', '{}'",
                first_rucksack, second_rucksack, third_rucksack
            ));
        }
    }
    Ok(AdventSolution::from(total_priority))
}

#[cfg(test)]
const DAY_03_SAMPLE_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp\n\
    jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n\
    PmmdzqPrVvPwwTWBwg\n\
    wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n\
    ttgJtRGJQctTZtZT\n\
    CrZsJsPPZsGzwwsLwLmpwMDw\n\n";

#[cfg(test)]
use crate::util::check_solution;

#[test]
fn test_part_1() {
    check_solution(&DAY_03_SAMPLE_INPUT, 157, &part_1);
}

#[test]
fn test_part_2() {
    check_solution(&DAY_03_SAMPLE_INPUT, 70, &part_2);
}
