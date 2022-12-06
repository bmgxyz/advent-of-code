use crate::util::{AdventResult, AdventSolution};

fn parse_trees(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| line.chars().map(|ch| ch == '#').collect())
        .collect()
}

fn count_trees(trees: &[Vec<bool>], rise: u8, run: u8) -> u64 {
    let (mut x, mut y) = (0, 0);
    let mut num_trees = 0;
    while y < trees.len() {
        if trees[y][x] {
            num_trees += 1;
        }
        x += run as usize;
        x %= trees[0].len();
        y += rise as usize;
    }
    num_trees
}

pub fn part_1(input: &str) -> AdventResult {
    Ok(AdventSolution::from(count_trees(&parse_trees(input), 1, 3)))
}

pub fn part_2(input: &str) -> AdventResult {
    let trees = parse_trees(input);
    Ok(AdventSolution::from(
        count_trees(&trees, 1, 1)
            * count_trees(&trees, 1, 3)
            * count_trees(&trees, 1, 5)
            * count_trees(&trees, 1, 7)
            * count_trees(&trees, 2, 1),
    ))
}

#[cfg(test)]
const DAY_03_SAMPLE_INPUT: &str = "..##.......\n\
    #...#...#..\n\
    .#....#..#.\n\
    ..#.#...#.#\n\
    .#...##..#.\n\
    ..#.##.....\n\
    .#.#.#....#\n\
    .#........#\n\
    #.##...#...\n\
    #...##....#\n\
    .#..#...#.#\n";

#[cfg(test)]
use crate::util::check_solution;

#[test]
fn test_part_1() {
    check_solution(&DAY_03_SAMPLE_INPUT.to_string(), 7, &part_1);
}

#[test]
fn test_part_2() {
    check_solution(&DAY_03_SAMPLE_INPUT.to_string(), 336, &part_2);
}
