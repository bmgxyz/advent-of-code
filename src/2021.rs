mod util;

pub fn solve_day_1_part_1(input: &String) -> Result<u64, String> {
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

pub fn solve_day_1_part_2(input: &String) -> Result<u64, String> {
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
}
