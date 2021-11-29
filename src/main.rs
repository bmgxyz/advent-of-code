use clap::{App, Arg};
use std::fs;

#[path = "2020.rs"]
mod solve_2020;

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
        Ok(y) if y >= 2015 && y <= 2021 => y,
        Ok(y) => {
            return Err(format!(
                "YEAR must be between 2015 and 2021, inclusive (got '{}')",
                y
            ))
        }
        Err(e) => return Err(format!("Failed to parse YEAR: {}", e)),
    };
    let day = match matches.value_of(DAY).unwrap().to_string().parse::<u8>() {
        Ok(d) if d >= 1 && d <= 25 => d,
        Ok(d) => {
            return Err(format!(
                "DAY must be between 2015 and 2021, inclusive (got '{}')",
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
            (d, p) => not_solved_yet(2015, d, p),
        },
        2016 => match (day, part) {
            (d, p) => not_solved_yet(2016, d, p),
        },
        2017 => match (day, part) {
            (d, p) => not_solved_yet(2017, d, p),
        },
        2018 => match (day, part) {
            (d, p) => not_solved_yet(2018, d, p),
        },
        2019 => match (day, part) {
            (d, p) => not_solved_yet(2019, d, p),
        },
        2020 => match (day, part) {
            (1, 1) => solve_2020::solve_day_1_part_1(&input),
            (1, 2) => solve_2020::solve_day_1_part_2(&input),
            (d, p) => not_solved_yet(2020, d, p),
        },
        2021 => match (day, part) {
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
        Err(e) => return Err(e),
    }
}

fn not_solved_yet(year: u16, day: u8, part: u8) -> Result<u64, String> {
    Err(format!(
        "{} day {} part {} isn't solved yet",
        year, day, part
    ))
}
