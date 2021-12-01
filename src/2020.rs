use regex::Regex;

mod util;

pub fn solve_day_1_part_1(input: &String) -> Result<u64, String> {
    let values = util::parse_u64(input);
    for a in values.iter() {
        for b in values.iter() {
            if a + b == 2020 {
                return Ok(a * b);
            }
        }
    }
    Err(String::from("Failed to find solution"))
}

pub fn solve_day_1_part_2(input: &String) -> Result<u64, String> {
    let values = util::parse_u64(input);
    for a in values.iter() {
        for b in values.iter() {
            for c in values.iter() {
                if a + b + c == 2020 {
                    return Ok(a * b * c);
                }
            }
        }
    }
    Err(String::from("Failed to find solution"))
}

pub fn solve_day_2_part_1(input: &String) -> Result<u64, String> {
    let mut valid_count = 0;
    let re =
        Regex::new(r"(?P<lower>\d+)-(?P<upper>\d+) (?P<key>[a-z]): (?P<password>[a-z]+)").unwrap();
    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        let ((lower_bound, upper_bound), key, password) = (
            (
                caps["lower"].parse::<u8>().unwrap(),
                caps["upper"].parse::<u8>().unwrap(),
            ),
            caps["key"].chars().next().unwrap(),
            caps["password"].to_string(),
        );
        let mut key_count = 0;
        for c in password.chars() {
            if c == key {
                key_count += 1;
            }
        }
        if lower_bound <= key_count && key_count <= upper_bound {
            valid_count += 1;
        }
    }
    Ok(valid_count)
}

pub fn solve_day_2_part_2(input: &String) -> Result<u64, String> {
    let mut valid_count = 0;
    let re =
        Regex::new(r"(?P<pos1>\d+)-(?P<pos2>\d+) (?P<key>[a-z]): (?P<password>[a-z]+)").unwrap();
    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        let ((pos1, pos2), key, password) = (
            (
                (caps["pos1"].parse::<u8>().unwrap() - 1) as usize,
                (caps["pos2"].parse::<u8>().unwrap() - 1) as usize,
            ),
            caps["key"].chars().next().unwrap(),
            caps["password"].to_string(),
        );
        if password.len() < pos2 + 1 {
            return Err(format!("Entry is too short: '{}'", input));
        }
        let pos1_char = password.chars().nth(pos1).unwrap();
        let pos2_char = password.chars().nth(pos2).unwrap();
        if (pos1_char == key && pos2_char != key) || (pos1_char != key && pos2_char == key) {
            valid_count += 1;
        }
    }
    Ok(valid_count)
}

#[cfg(test)]
mod test {
    use super::*;
    use util::check_solution;

    const DAY_1_SAMPLE_INPUT: &str = "1721\n\
        979\n\
        366\n\
        299\n\
        675\n\
        1456\n";

    #[test]
    fn day_1_part_1() {
        check_solution(&DAY_1_SAMPLE_INPUT.to_string(), 514579, &solve_day_1_part_1);
    }

    #[test]
    fn day_1_part_2() {
        check_solution(
            &DAY_1_SAMPLE_INPUT.to_string(),
            241861950,
            &solve_day_1_part_2,
        );
    }

    const DAY_2_SAMPLE_INPUT: &str = "1-3 a: abcde\n\
        1-3 b: cdefg\n\
        2-9 c: ccccccccc\n";

    #[test]
    fn day_2_part_1() {
        check_solution(&DAY_2_SAMPLE_INPUT.to_string(), 2, &solve_day_2_part_1);
    }

    #[test]
    fn day_2_part_2() {
        check_solution(&DAY_2_SAMPLE_INPUT.to_string(), 1, &solve_day_2_part_2);
    }
}
