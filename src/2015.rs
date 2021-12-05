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
}
