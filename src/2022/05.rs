use regex::Regex;

use crate::util::{AdventResult, AdventSolution};

struct CraneMove {
    from: usize,
    to: usize,
    quantity: usize,
}

fn parse_stacks(input: &str) -> Result<Vec<Vec<char>>, String> {
    // Get number of stacks by looking at the label row.
    let stack_label_re = Regex::new(r"^[\s\d]+$").unwrap();
    let mut num_stacks = 0;
    for line in input.lines() {
        if stack_label_re.is_match(line) {
            num_stacks = line
                .chars()
                .nth_back(1)
                .unwrap()
                .to_string()
                .parse()
                .unwrap();
        }
    }
    if num_stacks == 0 {
        return Err("Failed to find stack label row".to_string());
    }

    // Assemble the stacks into character vectors.
    let mut stacks = vec![vec![]; num_stacks];
    for line in input.lines() {
        if stack_label_re.is_match(line) {
            break;
        }
        for (idx, crate_letter) in line.chars().skip(1).step_by(4).enumerate() {
            match crate_letter {
                ' ' => (),
                c if c.is_ascii_uppercase() => stacks[idx].insert(0, c),
                _ => return Err(format!("Found invalid crate letter '{}'", crate_letter)),
            }
        }
    }

    Ok(stacks)
}

fn parse_crane_moves(input: &str) -> Vec<CraneMove> {
    let mut crane_moves = Vec::new();
    let move_re = Regex::new(r"^move (\d+) from (\d+) to (\d+)").unwrap();
    for line in input.lines() {
        match move_re.captures(line) {
            Some(caps) => {
                let (quantity, from, to) = (
                    caps[1].parse::<usize>().unwrap(),
                    caps[2].parse::<usize>().unwrap() - 1,
                    caps[3].parse::<usize>().unwrap() - 1,
                );
                crane_moves.push(CraneMove { from, to, quantity });
            }
            None => continue,
        }
    }
    crane_moves
}

pub fn part_1(input: &str) -> AdventResult {
    let mut stacks = parse_stacks(input)?;
    let crane_moves = parse_crane_moves(input);

    for crane_move in crane_moves {
        let CraneMove { from, to, quantity } = crane_move;
        for _ in 0..quantity {
            let crate_to_move = match stacks[from].pop() {
                Some(c) => c,
                None => continue,
            };
            stacks[to].push(crate_to_move);
        }
    }

    let message = stacks.iter().map(|s| s.last().unwrap()).collect::<String>();
    Ok(AdventSolution::from(message))
}

pub fn part_2(input: &str) -> AdventResult {
    let mut stacks = parse_stacks(input)?;
    let crane_moves = parse_crane_moves(input);

    for crane_move in crane_moves {
        let CraneMove { from, to, quantity } = crane_move;
        let mut crates_to_move = stacks[from][stacks[from].len() - quantity..].to_vec();
        stacks[from] = stacks[from][..stacks[from].len() - quantity].to_vec();
        stacks[to].append(&mut crates_to_move);
    }

    let message = stacks.iter().map(|s| s.last().unwrap()).collect::<String>();
    Ok(AdventSolution::from(message))
}

#[cfg(test)]
const SAMPLE_INPUT: &str = "    [D]    \n\
    [N] [C]    \n\
    [Z] [M] [P]\n 1   2   3 \n\
    \n\
    move 1 from 2 to 1\n\
    move 3 from 1 to 3\n\
    move 2 from 2 to 1\n\
    move 1 from 1 to 2\n";

#[cfg(test)]
use crate::util::check_solution;

#[test]
fn test_part_1() {
    check_solution(SAMPLE_INPUT, "CMZ", &part_1);
}

#[test]
fn test_part_2() {
    check_solution(SAMPLE_INPUT, "MCD", &part_2);
}
