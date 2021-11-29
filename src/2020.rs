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

#[test]
fn day_1_part_1() {
    let input = String::from(
        "1721\n\
        979\n\
        366\n\
        299\n\
        675\n\
        1456\n",
    );
    let solution = solve_day_1_part_1(&input);
    assert!(solution.is_ok());
    assert_eq!(solution.unwrap(), 514579);
}

#[test]
fn day_1_part_2() {
    let input = String::from(
        "1721\n\
        979\n\
        366\n\
        299\n\
        675\n\
        1456\n",
    );
    let solution = solve_day_1_part_2(&input);
    assert!(solution.is_ok());
    assert_eq!(solution.unwrap(), 241861950);
}
