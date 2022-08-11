use clap::{App, Arg};
use std::fs;

#[path = "2015.rs"]
mod solve_2015;

#[path = "2020.rs"]
mod solve_2020;

#[path = "2021.rs"]
mod solve_2021;

mod util;
use util::not_solved_yet;

fn main() -> Result<(), String> {
    // parse command line arguments
    // advent-of-code <YEAR> <DAY> <PART> <INPUT>
    const YEAR: &str = "YEAR";
    const DAY: &str = "DAY";
    const INPUT: &str = "INPUT";
    const PART: &str = "PART";
    let matches = App::new("Advent of Code")
        .version("0.1.0")
        .author("Bradley Gannon <bradley@bradleygannon.com>")
        .arg(
            Arg::with_name(YEAR)
                .help("Must be between 2015 and 2021, inclusive")
                .required(true),
        )
        .arg(
            Arg::with_name(DAY)
                .help("Must be between 1 and 25, inclusive")
                .required(true),
        )
        .arg(Arg::with_name(PART).help("Must be 1 or 2").required(true))
        .arg(
            Arg::with_name(INPUT)
                .help("Path to file containing puzzle input")
                .required(true),
        )
        .get_matches();

    // convert YEAR, DAY, and PART to numbers and check bounds
    let year = match matches.value_of(YEAR).unwrap().to_string().parse::<u16>() {
        Ok(y) if (2015..=2021).contains(&y) => y,
        Ok(y) => {
            return Err(format!(
                "YEAR must be between 2015 and 2021, inclusive (got '{}')",
                y
            ))
        }
        Err(e) => return Err(format!("Failed to parse YEAR: {}", e)),
    };
    let day = match matches.value_of(DAY).unwrap().to_string().parse::<u8>() {
        Ok(d) if (1..=25).contains(&d) => d,
        Ok(d) => {
            return Err(format!(
                "DAY must be between 1 and 25, inclusive (got '{}')",
                d
            ))
        }
        Err(e) => return Err(format!("Failed to parse DAY: {}", e)),
    };
    let part = match matches.value_of(PART).unwrap().to_string().parse::<u8>() {
        Ok(p) if p == 1 || p == 2 => p,
        Ok(p) => return Err(format!("PART must 1 or 2 (got '{}')", p)),
        Err(e) => return Err(format!("Failed to parse PART: {}", e)),
    };

    // read the input file
    let input = match fs::read_to_string(matches.value_of(INPUT).unwrap()) {
        Ok(i) => i,
        Err(e) => return Err(format!("Failed to read input file: {}", e)),
    };

    // pass the puzzle input to the solution function
    let solution = match year {
        2015 => match (day, part) {
            (1, 1) => solve_2015::solve_day_01_part_1(&input),
            (1, 2) => solve_2015::solve_day_01_part_2(&input),
            (2, 1) => solve_2015::solve_day_02_part_1(&input),
            (2, 2) => solve_2015::solve_day_02_part_2(&input),
            (3, 1) => solve_2015::solve_day_03_part_1(&input),
            (3, 2) => solve_2015::solve_day_03_part_2(&input),
            (4, 1) => solve_2015::solve_day_04_part_1(&input),
            (4, 2) => solve_2015::solve_day_04_part_2(&input),
            (5, 1) => solve_2015::solve_day_05_part_1(&input),
            (6, 1) => solve_2015::solve_day_06_part_1(&input),
            (6, 2) => solve_2015::solve_day_06_part_2(&input),
            (10, 1) => solve_2015::solve_day_10_part_1(&input),
            (12, 1) => solve_2015::solve_day_12_part_1(&input),
            (25, 1) => solve_2015::solve_day_25_part_1(&input),
            (d, p) => not_solved_yet(2015, d, p),
        },
        2016 => not_solved_yet(2016, day, part),
        2017 => not_solved_yet(2017, day, part),
        2018 => not_solved_yet(2018, day, part),
        2019 => not_solved_yet(2019, day, part),
        2020 => match (day, part) {
            (1, 1) => solve_2020::solve_day_01_part_1(&input),
            (1, 2) => solve_2020::solve_day_01_part_2(&input),
            (2, 1) => solve_2020::solve_day_02_part_1(&input),
            (2, 2) => solve_2020::solve_day_02_part_2(&input),
            (3, 1) => solve_2020::solve_day_03_part_1(&input),
            (3, 2) => solve_2020::solve_day_03_part_2(&input),
            (25, 1) => solve_2020::solve_day_25_part_1(&input),
            (d, p) => not_solved_yet(2020, d, p),
        },
        2021 => match (day, part) {
            (1, 1) => solve_2021::solve_day_01_part_1(&input),
            (1, 2) => solve_2021::solve_day_01_part_2(&input),
            (2, 1) => solve_2021::solve_day_02_part_1(&input),
            (2, 2) => solve_2021::solve_day_02_part_2(&input),
            (3, 1) => solve_2021::solve_day_03_part_1(&input),
            (3, 2) => solve_2021::solve_day_03_part_2(&input),
            (4, 1) => solve_2021::solve_day_04_part_1(&input),
            (4, 2) => solve_2021::solve_day_04_part_2(&input),
            (5, 1) => solve_2021::solve_day_05_part_1(&input),
            (5, 2) => solve_2021::solve_day_05_part_2(&input),
            (6, 1) => solve_2021::solve_day_06_part_1(&input),
            (6, 2) => solve_2021::solve_day_06_part_2(&input),
            (d, p) => not_solved_yet(2021, d, p),
        },
        _ => unreachable!(),
    };

    // print the solution
    match solution {
        Ok(s) => {
            println!("{}", s);
            Ok(())
        }
        Err(e) => Err(e),
    }
}
