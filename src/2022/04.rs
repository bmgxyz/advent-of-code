use regex::Regex;

use crate::util::AdventResult;

struct ElfPair {
    first_start: u64,
    first_end: u64,
    second_start: u64,
    second_end: u64,
}

impl ElfPair {
    fn is_complete_overlap(&self) -> bool {
        (self.first_start >= self.second_start && self.first_end <= self.second_end)
            || (self.second_start >= self.first_start && self.second_end <= self.first_end)
    }
    fn is_partial_overlap(&self) -> bool {
        (self.second_start <= self.first_start && self.first_start <= self.second_end)
            || (self.second_start <= self.first_end && self.first_end <= self.second_end)
            || self.is_complete_overlap()
    }
}

fn parse_elf_pairs(pairs: &str) -> Result<Vec<ElfPair>, String> {
    let mut elf_pairs = Vec::new();
    let re = Regex::new(r"(\d+)\-(\d+),(\d+)\-(\d+)").unwrap();
    for pair in pairs.lines() {
        if pair.is_empty() {
            continue;
        }
        match re.captures(pair) {
            Some(caps) => {
                let (first_start, first_end, second_start, second_end) = (
                    caps[1].parse::<u64>().unwrap(),
                    caps[2].parse::<u64>().unwrap(),
                    caps[3].parse::<u64>().unwrap(),
                    caps[4].parse::<u64>().unwrap(),
                );
                elf_pairs.push(ElfPair {
                    first_start,
                    first_end,
                    second_start,
                    second_end,
                });
            }
            None => return Err(format!("Found invalid pair: '{}'", pair)),
        }
    }
    Ok(elf_pairs)
}

pub fn part_1(input: &str) -> AdventResult {
    let pairs = parse_elf_pairs(input)?;
    Ok(pairs.iter().filter(|p| p.is_complete_overlap()).count() as u64)
}

pub fn part_2(input: &str) -> AdventResult {
    let pairs = parse_elf_pairs(input)?;
    Ok(pairs.iter().filter(|p| p.is_partial_overlap()).count() as u64)
}

#[cfg(test)]
const DAY_04_SAMPLE_INPUT: &str = "2-4,6-8\n\
    2-3,4-5\n\
    5-7,7-9\n\
    2-8,3-7\n\
    6-6,4-6\n\
    2-6,4-8\n";

#[cfg(test)]
use crate::util::check_solution;

#[test]
fn test_part_1() {
    check_solution(&DAY_04_SAMPLE_INPUT, 2, &part_1);
}

#[test]
fn test_part_2() {
    check_solution(&DAY_04_SAMPLE_INPUT, 4, &part_2);
}
