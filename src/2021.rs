mod util;

pub fn solve_day_1_part_1(input: &str) -> Result<u64, String> {
    let depths = util::parse_u64(input);
    if depths.len() < 2 {
        return Err(format!(
            "Not enough depth measurements: found {}",
            depths.len()
        ));
    }
    let mut increase_count = 0;
    let mut last_depth = depths[0];
    for depth in depths[1..].iter() {
        if *depth > last_depth {
            increase_count += 1;
        }
        last_depth = *depth;
    }
    Ok(increase_count)
}

pub fn solve_day_1_part_2(input: &str) -> Result<u64, String> {
    let depths = util::parse_u64(input);
    if depths.len() < 4 {
        return Err(format!(
            "Not enough depth measurements: found {}",
            depths.len()
        ));
    }
    let mut increase_count = 0;
    let mut last_measurement = depths[0] + depths[1] + depths[2];
    let mut new_measurement: u64;
    for i in 3..depths.len() {
        new_measurement = last_measurement - depths[i - 3] + depths[i];
        if new_measurement > last_measurement {
            increase_count += 1;
        }
        last_measurement = new_measurement;
    }
    Ok(increase_count)
}

pub fn solve_day_2_part_1(input: &str) -> Result<u64, String> {
    let mut horizontal_position = 0;
    let mut depth = 0;
    for line in input.lines() {
        let (direction, amount) = {
            let s: Vec<&str> = line.split(' ').collect();
            (s[0], s[1].parse::<u64>().unwrap())
        };
        match direction {
            "forward" => horizontal_position += amount,
            "up" => depth -= amount,
            "down" => depth += amount,
            _ => unreachable!(),
        };
    }
    Ok(horizontal_position * depth)
}

pub fn solve_day_2_part_2(input: &str) -> Result<u64, String> {
    let mut aim = 0;
    let mut depth = 0;
    let mut horizontal_position = 0;
    for line in input.lines() {
        let (direction, amount) = {
            let s: Vec<&str> = line.split(' ').collect();
            (s[0], s[1].parse::<u64>().unwrap())
        };
        match direction {
            "forward" => {
                horizontal_position += amount;
                depth += aim * amount;
            }
            "up" => aim -= amount,
            "down" => aim += amount,
            _ => unreachable!(),
        };
    }
    Ok(horizontal_position * depth)
}

pub fn solve_day_3_part_1(input: &str) -> Result<u64, String> {
    let length = input.lines().next().unwrap().len();
    let mut bit_counts = vec![0; length];
    for line in input.lines() {
        for (idx, ch) in line.chars().enumerate() {
            if ch == '1' {
                bit_counts[idx] += 1;
            } else {
                bit_counts[idx] -= 1;
            }
        }
    }
    let most_common_bits: Vec<bool> = bit_counts.iter().map(|b| b > &0).collect();
    let (gamma_rate, epsilon_rate) = {
        let (mut gamma_rate, mut epsilon_rate) = (0, 0);
        for (idx, bit) in most_common_bits.iter().rev().enumerate() {
            if *bit {
                gamma_rate += u64::pow(2, idx as u32);
            } else {
                epsilon_rate += u64::pow(2, idx as u32);
            }
        }
        (gamma_rate, epsilon_rate)
    };
    Ok(gamma_rate * epsilon_rate)
}

enum Gas {
    Oxygen,
    CarbonDioxide,
}

fn filter_numbers(numbers: &[&str], gas: Gas) -> String {
    let mut remaining_numbers = numbers.to_owned();
    let length = remaining_numbers[0].len();
    let mut bit_pos = 0;
    let mut bit_count;
    let mut digit: char;
    while remaining_numbers.len() > 1 && bit_pos < length {
        bit_count = 0;
        for number in remaining_numbers.iter() {
            if number.chars().nth(bit_pos).unwrap() == '1' {
                bit_count += 1;
            } else {
                bit_count -= 1;
            }
        }
        let mut idx = 0;
        while idx < remaining_numbers.len() {
            digit = remaining_numbers[idx].chars().nth(bit_pos).unwrap();
            match gas {
                Gas::Oxygen => {
                    if bit_count >= 0 && digit == '0' || bit_count < 0 && digit == '1' {
                        remaining_numbers.remove(idx);
                    } else {
                        idx += 1;
                    }
                }
                Gas::CarbonDioxide => {
                    if bit_count >= 0 && digit == '1' || bit_count < 0 && digit == '0' {
                        remaining_numbers.remove(idx);
                    } else {
                        idx += 1;
                    }
                }
            }
        }
        bit_pos += 1;
    }
    remaining_numbers[0].to_string()
}

pub fn solve_day_3_part_2(input: &str) -> Result<u64, String> {
    let remaining_numbers: Vec<&str> = input.lines().collect();
    let oxygen_rating = filter_numbers(&remaining_numbers, Gas::Oxygen);
    let carbon_dioxide_rating = filter_numbers(&remaining_numbers, Gas::CarbonDioxide);
    Ok(u64::from_str_radix(&oxygen_rating, 2).unwrap()
        * u64::from_str_radix(&carbon_dioxide_rating, 2).unwrap())
}

#[cfg(test)]
mod test {
    use super::*;
    use util::check_solution;

    const DAY_1_SAMPLE_INPUT: &str = "199\n\
        200\n\
        208\n\
        210\n\
        200\n\
        207\n\
        240\n\
        269\n\
        260\n\
        263\n";

    #[test]
    fn day_1_part_1() {
        check_solution(&DAY_1_SAMPLE_INPUT.to_string(), 7, &solve_day_1_part_1);
    }

    #[test]
    fn day_1_part_2() {
        check_solution(&DAY_1_SAMPLE_INPUT.to_string(), 5, &solve_day_1_part_2);
    }

    const DAY_2_SAMPLE_INPUT: &str = "forward 5\n\
        down 5\n\
        forward 8\n\
        up 3\n\
        down 8\n\
        forward 2\n";

    #[test]
    fn day_2_part_1() {
        check_solution(&DAY_2_SAMPLE_INPUT.to_string(), 150, &solve_day_2_part_1);
    }

    #[test]
    fn day_2_part_2() {
        check_solution(&DAY_2_SAMPLE_INPUT.to_string(), 900, &solve_day_2_part_2);
    }

    const DAY_3_SAMPLE_INPUT: &str = "00100\n\
        11110\n\
        10110\n\
        10111\n\
        10101\n\
        01111\n\
        00111\n\
        11100\n\
        10000\n\
        11001\n\
        00010\n\
        01010\n";

    #[test]
    fn day_3_part_1() {
        check_solution(&DAY_3_SAMPLE_INPUT.to_string(), 198, &solve_day_3_part_1);
    }

    #[test]
    fn day_3_part_2() {
        check_solution(&DAY_3_SAMPLE_INPUT.to_string(), 230, &solve_day_3_part_2);
    }
}
