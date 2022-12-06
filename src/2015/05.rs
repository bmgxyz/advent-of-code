use crate::util::{AdventResult, AdventSolution};

pub fn part_1(input: &str) -> AdventResult {
    let mut nice_count = 0;
    'lines: for line in input.lines() {
        if line.contains("ab") || line.contains("cd") || line.contains("pq") || line.contains("xy")
        {
            continue;
        }

        let vowel_count = line
            .chars()
            .filter(|c| *c == 'a' || *c == 'e' || *c == 'i' || *c == 'o' || *c == 'u')
            .count();
        if vowel_count < 3 {
            continue;
        }

        // Compare character pairs.
        let mut chars_iter = line.chars();
        let mut prev_chr = chars_iter.next().unwrap();
        for chr in chars_iter {
            if prev_chr == chr {
                nice_count += 1;
                continue 'lines;
            }
            prev_chr = chr;
        }
    }
    Ok(AdventSolution::from(nice_count))
}

pub fn part_2(input: &str) -> AdventResult {
    let mut nice_count = 0;
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let mut has_valid_pair = false;
        let mut has_repeat_sandwich = false;
        let chars = line.chars().collect::<Vec<char>>();
        'pairs: for outer in 0..chars.len() - 1 {
            let (outer_first, outer_second) = (chars[outer], chars[outer + 1]);
            for inner in 0..chars.len() - 1 {
                if outer.saturating_sub(1) <= inner && inner <= outer + 1 {
                    continue;
                }
                let (inner_first, inner_second) = (chars[inner], chars[inner + 1]);
                if outer_first == inner_first && outer_second == inner_second {
                    has_valid_pair = true;
                    break 'pairs;
                }
            }
        }
        // The next few lines could be more compact, but I think it's more clear
        // this way.
        if !has_valid_pair {
            continue;
        }
        for idx in 0..chars.len() - 2 {
            if chars[idx] == chars[idx + 2] {
                has_repeat_sandwich = true;
            }
        }
        if has_valid_pair && has_repeat_sandwich {
            nice_count += 1;
        }
    }
    Ok(AdventSolution::from(nice_count))
}

#[cfg(test)]
use crate::util::check_solution;

#[test]
fn test_part_1() {
    check_solution("ugknbfddgicrmopn\n", 1, &part_1);
    check_solution("aaa\n", 1, &part_1);
    check_solution("jchzalrnumimnmhp\n", 0, &part_1);
    check_solution("haegwjzuvuyypxyu\n", 0, &part_1);
    check_solution("dvszwmarrgswjxmb\n", 0, &part_1);
}

#[test]
fn test_part_2() {
    check_solution("qjhvhtzxzqqjkmpb", 1, &part_2);
    check_solution("xxyxx", 1, &part_2);
    check_solution("uurcxstgmygtbstg", 0, &part_2);
    check_solution("ieodomkazucvgmuy", 0, &part_2);
}
