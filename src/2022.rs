use crate::util::AdventResult;

fn parse_elf_calories(input: &str) -> Result<Vec<u64>, String> {
    let mut elf_calories = Vec::new();
    let mut current_total = 0;
    for line in input.lines() {
        match line {
            // A blank line indicates the end of an elf's inventory.
            "" => {
                elf_calories.push(current_total);
                current_total = 0;
            }
            _ => {
                current_total += match line.parse::<u64>() {
                    Ok(l) => l,
                    Err(e) => return Err(format!("Failed to parse calories '{}': {}", line, e)),
                }
            }
        }
    }
    Ok(elf_calories)
}

pub fn solve_day_01_part_1(input: &str) -> AdventResult {
    let elf_calories = parse_elf_calories(input)?;
    match elf_calories.iter().max() {
        Some(m) => Ok(*m),
        None => Err("No calories found".to_string()),
    }
}

pub fn solve_day_01_part_2(input: &str) -> AdventResult {
    let mut elf_calories = parse_elf_calories(input)?;
    elf_calories.sort();
    if elf_calories.len() >= 3 {
        Ok(elf_calories.iter().rev().take(3).sum())
    } else {
        Err("Not enough elves".to_string())
    }
}

#[derive(Clone, Copy)]
enum RpsChoice {
    Rock,
    Paper,
    Scissors,
}

fn compute_rps_score(rounds: Vec<(RpsChoice, RpsChoice)>) -> u64 {
    let mut score = 0;
    for (their_move, our_move) in rounds {
        score += match our_move {
            RpsChoice::Rock => 1,
            RpsChoice::Paper => 2,
            RpsChoice::Scissors => 3,
        };
        score += match (their_move, our_move) {
            // Win
            (RpsChoice::Scissors, RpsChoice::Rock)
            | (RpsChoice::Paper, RpsChoice::Scissors)
            | (RpsChoice::Rock, RpsChoice::Paper) => 6,
            // Draw
            (RpsChoice::Rock, RpsChoice::Rock)
            | (RpsChoice::Paper, RpsChoice::Paper)
            | (RpsChoice::Scissors, RpsChoice::Scissors) => 3,
            // Loss (all other cases)
            (_, _) => 0,
        };
    }
    score
}

pub fn solve_day_02_part_1(input: &str) -> AdventResult {
    let mut guide = Vec::new();
    for line in input.lines() {
        if line.len() >= 3 {
            let their_move = match line.chars().next().unwrap() {
                'A' => RpsChoice::Rock,
                'B' => RpsChoice::Paper,
                'C' => RpsChoice::Scissors,
                c => return Err(format!("Found unexpected character '{}'", c)),
            };
            let our_move = match line.chars().nth(2).unwrap() {
                'X' => RpsChoice::Rock,
                'Y' => RpsChoice::Paper,
                'Z' => RpsChoice::Scissors,
                c => return Err(format!("Found unexpected character '{}'", c)),
            };
            guide.push((their_move, our_move));
        }
    }
    Ok(compute_rps_score(guide))
}

pub fn solve_day_02_part_2(input: &str) -> AdventResult {
    let mut guide = Vec::new();
    for line in input.lines() {
        if line.len() >= 3 {
            let their_move = match line.chars().next().unwrap() {
                'A' => RpsChoice::Rock,
                'B' => RpsChoice::Paper,
                'C' => RpsChoice::Scissors,
                c => return Err(format!("Found unexpected character '{}'", c)),
            };
            let our_move = match line.chars().nth(2).unwrap() {
                // Loss
                'X' => match their_move {
                    RpsChoice::Rock => RpsChoice::Scissors,
                    RpsChoice::Paper => RpsChoice::Rock,
                    RpsChoice::Scissors => RpsChoice::Paper,
                },
                // Draw
                'Y' => their_move,
                // Win
                'Z' => match their_move {
                    RpsChoice::Rock => RpsChoice::Paper,
                    RpsChoice::Paper => RpsChoice::Scissors,
                    RpsChoice::Scissors => RpsChoice::Rock,
                },
                c => return Err(format!("Found unexpected character '{}'", c)),
            };
            guide.push((their_move, our_move));
        }
    }
    Ok(compute_rps_score(guide))
}

fn compute_item_priority(item: char) -> AdventResult {
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

pub fn solve_day_03_part_1(input: &str) -> AdventResult {
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
    Ok(total_priority)
}

pub fn solve_day_03_part_2(input: &str) -> AdventResult {
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
    Ok(total_priority)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::util::check_solution;

    const DAY_01_SAMPLE_INPUT: &str = "1000\n\
        2000\n\
        3000\n\
        \n\
        4000\n\
        \n\
        5000\n\
        6000\n\
        \n\
        7000\n\
        8000\n\
        9000\n\
        \n\
        10000\n\n";

    #[test]
    fn day_01_part_1() {
        check_solution(
            &DAY_01_SAMPLE_INPUT.to_string(),
            24000,
            &solve_day_01_part_1,
        );
    }

    #[test]
    fn day_01_part_2() {
        check_solution(
            &DAY_01_SAMPLE_INPUT.to_string(),
            45000,
            &solve_day_01_part_2,
        );
    }

    const DAY_02_SAMPLE_INPUT: &str = "A Y\n\
        B X\n\
        C Z\n";

    #[test]
    fn day_02_part_1() {
        check_solution(&DAY_02_SAMPLE_INPUT.to_string(), 15, &solve_day_02_part_1);
    }

    #[test]
    fn day_02_part_2() {
        check_solution(&DAY_02_SAMPLE_INPUT.to_string(), 12, &solve_day_02_part_2);
    }

    const DAY_03_SAMPLE_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp\n\
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n\
        PmmdzqPrVvPwwTWBwg\n\
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n\
        ttgJtRGJQctTZtZT\n\
        CrZsJsPPZsGzwwsLwLmpwMDw\n\n";

    #[test]
    fn day_03_part_1() {
        check_solution(&DAY_03_SAMPLE_INPUT, 157, &solve_day_03_part_1);
    }

    #[test]
    fn day_03_part_2() {
        check_solution(&DAY_03_SAMPLE_INPUT, 70, &solve_day_03_part_2);
    }
}
