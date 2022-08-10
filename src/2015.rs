use regex::Regex;

mod util;

pub fn solve_day_01_part_1(input: &str) -> Result<u64, String> {
    let mut floor: i64 = 0;
    for ch in input.chars() {
        if ch == '(' {
            floor += 1;
        } else if ch == ')' {
            floor -= 1;
        }
    }
    Ok(floor as u64)
}

pub fn solve_day_01_part_2(input: &str) -> Result<u64, String> {
    let mut floor = 0;
    for (idx, ch) in input.chars().enumerate() {
        if ch == '(' {
            floor += 1;
        } else if ch == ')' {
            floor -= 1;
        }
        if floor < 0 {
            return Ok((idx as u64) + 1);
        }
    }
    Err(String::from("Failed to find the basement"))
}

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

pub fn solve_day_02_part_1(input: &str) -> Result<u64, String> {
    let mut dimensions = parse_day_02_input(input);
    let mut paper_total = 0;
    for dim in dimensions.iter_mut() {
        dim.sort_unstable();
        paper_total +=
            2 * dim[0] * dim[1] + 2 * dim[1] * dim[2] + 2 * dim[2] * dim[0] + dim[0] * dim[1];
    }
    Ok(paper_total)
}

pub fn solve_day_02_part_2(input: &str) -> Result<u64, String> {
    let mut dimensions = parse_day_02_input(input);
    let mut ribbon_total = 0;
    for dim in dimensions.iter_mut() {
        dim.sort_unstable();
        ribbon_total += 2 * dim[0] + 2 * dim[1] + dim[0] * dim[1] * dim[2];
    }
    Ok(ribbon_total)
}

pub fn solve_day_03_part_1(input: &str) -> Result<u64, String> {
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
    Ok(visited.len() as u64)
}

enum Actor {
    Santa,
    Robot,
}

/// This solution is due to Brady Butler (https://github.com/mbbutler).
pub fn solve_day_03_part_2(input: &str) -> Result<u64, String> {
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
    Ok(visited.len() as u64)
}

pub fn solve_day_05_part_1(input: &str) -> Result<u64, String> {
    let mut nice_count = 0;
    'lines: for line in input.lines() {
        if line.contains("ab") || line.contains("cd") || line.contains("pq") || line.contains("xy")
        {
            continue;
        }

        let vowel_count = line
            .chars()
            .filter(|c| *c == 'a' || *c == 'e' || *c == 'i' || *c == 'o' || *c == 'u')
            .count();
        if vowel_count < 3 {
            continue;
        }

        // Compare character pairs.
        let mut chars_iter = line.chars();
        let mut prev_chr = chars_iter.next().unwrap();
        while let Some(chr) = chars_iter.next() {
            if prev_chr == chr {
                nice_count += 1;
                continue 'lines;
            }
            prev_chr = chr;
        }
    }
    Ok(nice_count)
}

pub fn solve_day_06_part_1(input: &str) -> Result<u64, String> {
    let re = Regex::new(r"(turn on|toggle|turn off) (\d+),(\d+) through (\d+),(\d+)").unwrap();
    let mut lights = vec![vec![false; 1000]; 1000];
    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        let (action, x1, y1, x2, y2) = (
            &caps[1],
            caps[2].parse::<usize>().unwrap(),
            caps[3].parse::<usize>().unwrap(),
            caps[4].parse::<usize>().unwrap(),
            caps[5].parse::<usize>().unwrap(),
        );
        match action {
            "turn on" => {
                for row in y1..=y2 {
                    for col in x1..=x2 {
                        lights[row][col] = true;
                    }
                }
            }
            "toggle" => {
                for row in y1..=y2 {
                    for col in x1..=x2 {
                        lights[row][col] = !lights[row][col];
                    }
                }
            }
            "turn off" => {
                for row in y1..=y2 {
                    for col in x1..=x2 {
                        lights[row][col] = false;
                    }
                }
            }
            _ => unreachable!(),
        };
    }
    let mut enabled_lights = 0;
    for row in lights {
        for light in row {
            if light {
                enabled_lights += 1;
            }
        }
    }
    Ok(enabled_lights)
}

pub fn solve_day_10_part_1(input: &str) -> Result<u64, String> {
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

pub fn solve_day_12_part_1(input: &str) -> Result<u64, String> {
    let mut sum = 0;
    let mut number = 0;
    let mut negative = false;
    for chr in input.chars() {
        if chr == '-' {
            negative = true;
        } else if let Ok(d) = chr.to_string().parse::<i64>() {
            number *= 10;
            if negative {
                number -= d;
            } else {
                number += d;
            }
        } else {
            sum += number;
            number = 0;
            negative = false;
        }
    }
    // Cast as u64 here because we can assume the result will be positive.
    Ok(sum as u64)
}

#[cfg(test)]
mod test {
    use super::*;
    use util::check_solution;

    #[test]
    fn day_01_part_1() {
        check_solution("(())\n", 0, &solve_day_01_part_1);
        check_solution("()()\n", 0, &solve_day_01_part_1);
        check_solution("(((\n", 3, &solve_day_01_part_1);
        check_solution("(()(()(\n", 3, &solve_day_01_part_1);
        check_solution("))(((((\n", 3, &solve_day_01_part_1);
    }

    #[test]
    fn day_01_part_2() {
        check_solution(")\n", 1, &solve_day_01_part_2);
        check_solution("()())\n", 5, &solve_day_01_part_2);
    }

    const DAY_02_SAMPLE_INPUT: &str = "2x3x4\n1x1x10\n";

    #[test]
    fn day_02_part_1() {
        check_solution(&DAY_02_SAMPLE_INPUT, 101, &solve_day_02_part_1);
    }

    #[test]
    fn day_02_part_2() {
        check_solution(&DAY_02_SAMPLE_INPUT, 48, &solve_day_02_part_2);
    }

    #[test]
    fn day_03_part_1() {
        check_solution(">", 2, &solve_day_03_part_1);
        check_solution("^>v<", 4, &solve_day_03_part_1);
        check_solution("^v^v^v^v^v", 2, &solve_day_03_part_1);
    }

    // #[test]
    // fn day_03_part_2() {
    //     // check_solution(input, output, solution);
    //     unimplemented!();
    // }

    #[test]
    fn day_05_part_1() {
        check_solution("ugknbfddgicrmopn\n", 1, &solve_day_05_part_1);
        check_solution("aaa\n", 1, &solve_day_05_part_1);
        check_solution("jchzalrnumimnmhp\n", 0, &solve_day_05_part_1);
        check_solution("haegwjzuvuyypxyu\n", 0, &solve_day_05_part_1);
        check_solution("dvszwmarrgswjxmb\n", 0, &solve_day_05_part_1);
    }

    #[test]
    fn day_06_part_1() {
        check_solution(
            "turn on 0,0 through 999,999\n",
            1_000_000,
            &solve_day_06_part_1,
        );
        check_solution(
            "turn on 0,0 through 999,999\ntoggle 0,0 through 999,0",
            999_000,
            &solve_day_06_part_1,
        );
        check_solution(
            "turn on 0,0 through 999,999\nturn off 499,499 through 500,500",
            999_996,
            &solve_day_06_part_1,
        );
    }

    #[test]
    fn day_10_part_1() {
        check_solution("1", 82_350, &solve_day_10_part_1);
    }

    #[test]
    fn day_12_part_1() {
        check_solution("[1,2,3]", 6, &solve_day_12_part_1);
        check_solution("{\"a\":2,\"b\":4}", 6, &solve_day_12_part_1);
        check_solution("[[[3]]]", 3, &solve_day_12_part_1);
        check_solution("{\"a\":{\"b\":4},\"c\":-1}", 3, &solve_day_12_part_1);
        check_solution("{\"a\":[-1,1]}", 0, &solve_day_12_part_1);
        check_solution("[-1,{\"a\":1}]", 0, &solve_day_12_part_1);
        check_solution("[]", 0, &solve_day_12_part_1);
        check_solution("{}", 0, &solve_day_12_part_1);

        // Extra multi-digit test case
        check_solution("{\"a\":1854,\"b\":-430}", 1_424, &solve_day_12_part_1);
    }
}
