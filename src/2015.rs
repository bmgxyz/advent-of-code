mod util;

pub fn solve_day_1_part_1(input: &str) -> Result<u64, String> {
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

pub fn solve_day_1_part_2(input: &str) -> Result<u64, String> {
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

#[cfg(test)]
mod test {
    use super::*;
    use util::check_solution;

    #[test]
    fn day_1_part_1() {
        check_solution("(())\n", 0, &solve_day_1_part_1);
        check_solution("()()\n", 0, &solve_day_1_part_1);
        check_solution("(((\n", 3, &solve_day_1_part_1);
        check_solution("(()(()(\n", 3, &solve_day_1_part_1);
        check_solution("))(((((\n", 3, &solve_day_1_part_1);
    }

    #[test]
    fn day_1_part_2() {
        check_solution(")\n", 1, &solve_day_1_part_2);
        check_solution("()())\n", 5, &solve_day_1_part_2);
    }
}
