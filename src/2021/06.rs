use crate::util::{AdventResult, AdventSolution};

fn simulate_lanternfish(input: &str, days: u16) -> u64 {
    let initial_lanternfish: Vec<u64> = input
        .split(',')
        .map(|f| f.trim_end().parse::<u64>().unwrap())
        .collect();
    let mut lanternfish = [0u64; 9];
    // count the fish in each timer state
    for fish in initial_lanternfish {
        lanternfish[fish as usize] += 1;
    }
    let mut zero_fish;
    for _ in 0..days {
        zero_fish = lanternfish[0];
        // subtract all fish timers by shifting values left
        for idx in 0..8 {
            lanternfish[idx] = lanternfish[idx + 1];
        }
        // all the fish that were at zero are now at six
        lanternfish[6] += zero_fish;
        // and they reproduce, which results in an equal number of fish at eight
        lanternfish[8] = zero_fish;
    }
    lanternfish.iter().sum::<u64>()
}

pub fn part_1(input: &str) -> AdventResult {
    Ok(AdventSolution::from(simulate_lanternfish(input, 80)))
}

pub fn part_2(input: &str) -> AdventResult {
    Ok(AdventSolution::from(simulate_lanternfish(input, 256)))
}

#[cfg(test)]
const DAY_06_SAMPLE_INPUT: &str = "3,4,3,1,2\n";

#[cfg(test)]
use crate::util::check_solution;

#[test]
fn test_part_1() {
    check_solution(&DAY_06_SAMPLE_INPUT, 5934, &part_1);
}

#[test]
fn test_part_2() {
    check_solution(&DAY_06_SAMPLE_INPUT, 26984457539_u64, &part_2);
}
