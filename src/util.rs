use std::fmt::{Debug, Display};

#[derive(Debug, PartialEq, Eq)]
pub struct AdventSolution(String);

macro_rules! from_for_advent_solution {
    ($from_type: ty) => {
        impl From<$from_type> for AdventSolution {
            fn from(x: $from_type) -> Self {
                AdventSolution(x.to_string())
            }
        }
    };
}

from_for_advent_solution!(i32);
from_for_advent_solution!(i64);
from_for_advent_solution!(u64);
from_for_advent_solution!(usize);
from_for_advent_solution!(&str);

impl From<String> for AdventSolution {
    fn from(s: String) -> Self {
        AdventSolution(s)
    }
}

impl Display for AdventSolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub type AdventResult = Result<AdventSolution, String>;

pub fn not_solved_yet(year: u16, day: u8, part: u8) -> AdventResult {
    Err(format!(
        "{} day {} part {} isn't solved yet",
        year, day, part
    ))
}

#[cfg(test)]
pub fn check_solution<T>(input: &str, output: T, solution: &dyn Fn(&str) -> AdventResult)
where
    T: Into<AdventSolution>,
{
    let solution_result = solution(input);
    assert!(solution_result.is_ok());
    let correct = output.into();
    assert_eq!(solution_result.unwrap(), correct);
}

pub fn parse_u64(input: &str) -> Vec<u64> {
    input
        .split_terminator('\n')
        .map(|x| x.parse::<u64>().unwrap())
        .collect()
}
