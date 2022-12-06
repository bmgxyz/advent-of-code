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
