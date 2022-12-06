use crate::util::{AdventResult, AdventSolution};

pub fn part_1(input: &str) -> AdventResult {
    let mut position = (0, 0);
    let mut visited = std::collections::HashSet::new();
    visited.insert((0, 0));
    for step in input.chars() {
        match step {
            '>' => position.0 += 1,
            '<' => position.0 -= 1,
            '^' => position.1 += 1,
            'v' => position.1 -= 1,
            c => return Err(format!("Found bad character: '{}'", c)),
        };
        visited.insert(position);
    }
    Ok(AdventSolution::from(visited.len()))
}

enum Actor {
    Santa,
    Robot,
}

/// This solution is due to Brady Butler (https://github.com/mbbutler).
pub fn part_2(input: &str) -> AdventResult {
    let mut santa = (0, 0);
    let mut robot = (0, 0);
    let mut visited = std::collections::HashSet::new();
    visited.insert((0, 0));
    let mut actor: Actor = Actor::Santa;
    for step in input.chars() {
        let mut position = match actor {
            Actor::Santa => {
                actor = Actor::Robot;
                &mut santa
            }
            Actor::Robot => {
                actor = Actor::Santa;
                &mut robot
            }
        };
        match step {
            '>' => position.0 += 1,
            '<' => position.0 -= 1,
            '^' => position.1 += 1,
            'v' => position.1 -= 1,
            c => return Err(format!("Found bad character: '{}'", c)),
        };
        visited.insert(*position);
    }
    Ok(AdventSolution::from(visited.len()))
}

#[cfg(test)]
use crate::util::check_solution;

#[test]
fn test_part_1() {
    check_solution(">", 2, &part_1);
    check_solution("^>v<", 4, &part_1);
    check_solution("^v^v^v^v^v", 2, &part_1);
}

#[test]
fn test_part_2() {
    check_solution("^v", 3, &part_2);
    check_solution("^>v<", 3, &part_2);
    check_solution("^v^v^v^v^v", 11, &part_2);
}
