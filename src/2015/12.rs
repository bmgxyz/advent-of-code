use crate::util::AdventResult;

pub fn part_1(input: &str) -> AdventResult {
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
use crate::util::check_solution;

#[test]
fn test_part_1() {
    check_solution("[1,2,3]", 6, &part_1);
    check_solution("{\"a\":2,\"b\":4}", 6, &part_1);
    check_solution("[[[3]]]", 3, &part_1);
    check_solution("{\"a\":{\"b\":4},\"c\":-1}", 3, &part_1);
    check_solution("{\"a\":[-1,1]}", 0, &part_1);
    check_solution("[-1,{\"a\":1}]", 0, &part_1);
    check_solution("[]", 0, &part_1);
    check_solution("{}", 0, &part_1);

    // Extra multi-digit test case
    check_solution("{\"a\":1854,\"b\":-430}", 1_424, &part_1);
}
