use crate::util::{AdventResult, AdventSolution};

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

pub fn part_1(input: &str) -> AdventResult {
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
    Ok(AdventSolution::from(compute_rps_score(guide)))
}

pub fn part_2(input: &str) -> AdventResult {
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
    Ok(AdventSolution::from(compute_rps_score(guide)))
}

#[cfg(test)]
const DAY_02_SAMPLE_INPUT: &str = "A Y\n\
    B X\n\
    C Z\n";

#[cfg(test)]
use crate::util::check_solution;

#[test]
fn test_part_1() {
    check_solution(&DAY_02_SAMPLE_INPUT.to_string(), 15, &part_1);
}

#[test]
fn test_part_2() {
    check_solution(&DAY_02_SAMPLE_INPUT.to_string(), 12, &part_2);
}
