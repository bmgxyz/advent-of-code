use std::collections::HashMap;

use regex::Regex;

use crate::util::{AdventResult, AdventSolution};

// Brady Butler (@mbbutler) and Logan Boyd (@loboyd) contributed to this
// solution.
pub fn part_1(input: &str) -> AdventResult {
    let mut pwd: Vec<String> = Vec::new();
    let mut dir_sizes = HashMap::new();
    let cd_re = Regex::new(r"^\$ cd (.+)$").unwrap();
    let file_re = Regex::new(r"^(\d+) (.+)$").unwrap();
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        if let Some(cap) = cd_re.captures(line) {
            let target_dir = &cap[1];
            if target_dir == ".." {
                pwd.pop();
            } else {
                pwd.push(target_dir.to_string());
            }
        }
        if let Some(cap) = file_re.captures(line) {
            let (file_size, _file_name) = (cap[1].parse::<u64>().unwrap(), &cap[2]);
            let pwd_clone = pwd.clone();
            for idx in 1..pwd.len() + 1 {
                let pwd_key: String = pwd_clone
                    .iter()
                    .take(idx)
                    .cloned()
                    .fold(String::new(), |acc, x| acc + &x);
                #[allow(clippy::map_entry)]
                if !dir_sizes.contains_key(&pwd_key.clone()) {
                    dir_sizes.insert(pwd_key.clone(), file_size);
                } else if let Some(d) = dir_sizes.get_mut(&pwd_key.clone()) {
                    *d += file_size;
                }
            }
        }
    }
    let mut total_small_dir_size = 0;
    for dir_size in dir_sizes.values() {
        if *dir_size <= 100_000 {
            total_small_dir_size += dir_size;
        }
    }
    Ok(AdventSolution::from(total_small_dir_size))
}

pub fn part_2(_input: &str) -> AdventResult {
    todo!();
}

#[cfg(test)]
use crate::util::check_solution;

#[cfg(test)]
const SAMPLE_INPUT: &str = "$ cd /\n\
    $ ls\n\
    dir a\n\
    14848514 b.txt\n\
    8504156 c.dat\n\
    dir d\n\
    $ cd a\n\
    $ ls\n\
    dir e\n\
    29116 f\n\
    2557 g\n\
    62596 h.lst\n\
    $ cd e\n\
    $ ls\n\
    584 i\n\
    $ cd ..\n\
    $ cd ..\n\
    $ cd d\n\
    $ ls\n\
    4060174 j\n\
    8033020 d.log\n\
    5626152 d.ext\n\
    7214296 k\n";

#[test]
fn test_part_1() {
    check_solution(SAMPLE_INPUT, 95437, &part_1);
}

// #[test]
// fn test_part_2() {
//     check_solution("", 0, &part_2);
// }
