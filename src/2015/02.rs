use crate::util::{AdventResult, AdventSolution};

fn parse_day_02_input(input: &str) -> Vec<Vec<u64>> {
    input
        .lines()
        .map(|line| {
            line.split('x')
                .map(|dim| dim.parse::<u64>().unwrap())
                .collect()
        })
        .collect()
}

pub fn part_1(input: &str) -> AdventResult {
    let mut dimensions = parse_day_02_input(input);
    let mut paper_total = 0;
    for dim in dimensions.iter_mut() {
        dim.sort_unstable();
        paper_total +=
            2 * dim[0] * dim[1] + 2 * dim[1] * dim[2] + 2 * dim[2] * dim[0] + dim[0] * dim[1];
    }
    Ok(AdventSolution::from(paper_total))
}

pub fn part_2(input: &str) -> AdventResult {
    let mut dimensions = parse_day_02_input(input);
    let mut ribbon_total = 0;
    for dim in dimensions.iter_mut() {
        dim.sort_unstable();
        ribbon_total += 2 * dim[0] + 2 * dim[1] + dim[0] * dim[1] * dim[2];
    }
    Ok(AdventSolution::from(ribbon_total))
}

#[cfg(test)]
const DAY_02_SAMPLE_INPUT: &str = "2x3x4\n1x1x10\n";

#[cfg(test)]
use crate::util::check_solution;

#[test]
fn test_part_1() {
    check_solution(&DAY_02_SAMPLE_INPUT, 101, &part_1);
}

#[test]
fn test_part_2() {
    check_solution(&DAY_02_SAMPLE_INPUT, 48, &part_2);
}
