use std::num::Wrapping;

use regex::Regex;

mod util;

pub fn solve_day_01_part_1(input: &str) -> Result<u64, String> {
    let mut floor: i64 = 0;
    for ch in input.chars() {
        if ch == '(' {
            floor += 1;
        } else if ch == ')' {
            floor -= 1;
        }
    }
    Ok(floor as u64)
}

pub fn solve_day_01_part_2(input: &str) -> Result<u64, String> {
    let mut floor = 0;
    for (idx, ch) in input.chars().enumerate() {
        if ch == '(' {
            floor += 1;
        } else if ch == ')' {
            floor -= 1;
        }
        if floor < 0 {
            return Ok((idx as u64) + 1);
        }
    }
    Err(String::from("Failed to find the basement"))
}

fn parse_day_02_input(input: &str) -> Vec<Vec<u64>> {
    input
        .lines()
        .map(|line| {
            line.split('x')
                .map(|dim| dim.parse::<u64>().unwrap())
                .collect()
        })
        .collect()
}

pub fn solve_day_02_part_1(input: &str) -> Result<u64, String> {
    let mut dimensions = parse_day_02_input(input);
    let mut paper_total = 0;
    for dim in dimensions.iter_mut() {
        dim.sort_unstable();
        paper_total +=
            2 * dim[0] * dim[1] + 2 * dim[1] * dim[2] + 2 * dim[2] * dim[0] + dim[0] * dim[1];
    }
    Ok(paper_total)
}

pub fn solve_day_02_part_2(input: &str) -> Result<u64, String> {
    let mut dimensions = parse_day_02_input(input);
    let mut ribbon_total = 0;
    for dim in dimensions.iter_mut() {
        dim.sort_unstable();
        ribbon_total += 2 * dim[0] + 2 * dim[1] + dim[0] * dim[1] * dim[2];
    }
    Ok(ribbon_total)
}

pub fn solve_day_03_part_1(input: &str) -> Result<u64, String> {
    let mut position = (0, 0);
    let mut visited = std::collections::HashSet::new();
    visited.insert((0, 0));
    for step in input.chars() {
        match step {
            '>' => position.0 += 1,
            '<' => position.0 -= 1,
            '^' => position.1 += 1,
            'v' => position.1 -= 1,
            c => return Err(format!("Found bad character: '{}'", c)),
        };
        visited.insert(position);
    }
    Ok(visited.len() as u64)
}

enum Actor {
    Santa,
    Robot,
}

/// This solution is due to Brady Butler (https://github.com/mbbutler).
pub fn solve_day_03_part_2(input: &str) -> Result<u64, String> {
    let mut santa = (0, 0);
    let mut robot = (0, 0);
    let mut visited = std::collections::HashSet::new();
    visited.insert((0, 0));
    let mut actor: Actor = Actor::Santa;
    for step in input.chars() {
        let mut position = match actor {
            Actor::Santa => {
                actor = Actor::Robot;
                &mut santa
            }
            Actor::Robot => {
                actor = Actor::Santa;
                &mut robot
            }
        };
        match step {
            '>' => position.0 += 1,
            '<' => position.0 -= 1,
            '^' => position.1 += 1,
            'v' => position.1 -= 1,
            c => return Err(format!("Found bad character: '{}'", c)),
        };
        visited.insert(*position);
    }
    Ok(visited.len() as u64)
}

/// MD5 implementation that only works for messages that are less than about 56 bytes long.
fn md5_short(message: &[u8]) -> u128 {
    const S: [usize; 64] = [
        7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20, 5,
        9, 14, 20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 6, 10,
        15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
    ];
    const K: [u32; 64] = [
        0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613,
        0xfd469501, 0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193,
        0xa679438e, 0x49b40821, 0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d,
        0x02441453, 0xd8a1e681, 0xe7d3fbc8, 0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed,
        0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a, 0xfffa3942, 0x8771f681, 0x6d9d6122,
        0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70, 0x289b7ec6, 0xeaa127fa,
        0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665, 0xf4292244,
        0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
        0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb,
        0xeb86d391,
    ];

    let (mut a0, mut b0, mut c0, mut d0) = (
        Wrapping(0x67452301u32),
        Wrapping(0xefcdab89u32),
        Wrapping(0x98badcfeu32),
        Wrapping(0x10325476u32),
    );

    let mut chunk = message.to_vec();
    chunk.push(0x80);
    while chunk.len() < 56 {
        chunk.push(0x00);
    }
    let message_len = (message.len() * 8).to_le_bytes();
    for b in message_len {
        chunk.push(b);
    }
    let words: Vec<u32> = chunk
        .chunks(4)
        .map(|w| w.iter().fold(0, |acc, b| (acc >> 8) + ((*b as u32) << 24)))
        .collect();

    let (mut a, mut b, mut c, mut d) = (a0, b0, c0, d0);
    for i in 0..64 {
        let (mut f, g) = match i {
            _ if (0..=15).contains(&i) => ((b & c) | (!b & d), i),
            _ if (16..=31).contains(&i) => ((d & b) | (!d & c), (5 * i + 1) % 16),
            _ if (32..=47).contains(&i) => (b ^ c ^ d, (3 * i + 5) % 16),
            _ if (48..=63).contains(&i) => (c ^ (b | !d), (7 * i) % 16),
            _ => unreachable!(),
        };
        f += a + Wrapping(K[i]) + Wrapping(words[g]);
        a = d;
        d = c;
        c = b;
        b += (f << S[i]) | (f >> (32 - S[i]));
    }
    a0 += a;
    b0 += b;
    c0 += c;
    d0 += d;

    let mut output = 0;
    for b in
        a0.0.to_le_bytes()
            .iter()
            .chain(b0.0.to_le_bytes().iter())
            .chain(c0.0.to_le_bytes().iter())
            .chain(d0.0.to_le_bytes().iter())
    {
        output <<= 8;
        output += *b as u128;
    }
    output
}

fn concat_message(original_message: &[u8], number: u64) -> Vec<u8> {
    let mut new_message = original_message.to_vec();
    for b in number.to_string().as_bytes() {
        new_message.push(*b);
    }
    new_message
}

pub fn solve_day_04_part_1(input: &str) -> Result<u64, String> {
    let mut number = 0;
    let original_message = input.as_bytes();
    // Run until the first 20 bits are zero.
    while md5_short(&concat_message(original_message, number)) >> 108 != 0 {
        number += 1;
    }
    Ok(number)
}

pub fn solve_day_04_part_2(input: &str) -> Result<u64, String> {
    let mut number = 0;
    let original_message = input.as_bytes();
    // Run until the first 24 bits are zero.
    while md5_short(&concat_message(original_message, number)) >> 104 != 0 {
        number += 1;
    }
    Ok(number)
}

pub fn solve_day_05_part_1(input: &str) -> Result<u64, String> {
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
    Ok(nice_count)
}

pub fn solve_day_06_part_1(input: &str) -> Result<u64, String> {
    let re = Regex::new(r"(turn on|toggle|turn off) (\d+),(\d+) through (\d+),(\d+)").unwrap();
    let mut lights = vec![vec![false; 1000]; 1000];
    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        let (action, x1, y1, x2, y2) = (
            &caps[1],
            caps[2].parse::<usize>().unwrap(),
            caps[3].parse::<usize>().unwrap(),
            caps[4].parse::<usize>().unwrap(),
            caps[5].parse::<usize>().unwrap(),
        );
        // I think it's more clear to explicitly index into lights rather than
        // construct an iterator over it.
        #[allow(clippy::needless_range_loop)]
        match action {
            "turn on" => {
                for row in y1..=y2 {
                    for col in x1..=x2 {
                        lights[row][col] = true;
                    }
                }
            }
            "toggle" => {
                for row in y1..=y2 {
                    for col in x1..=x2 {
                        lights[row][col] = !lights[row][col];
                    }
                }
            }
            "turn off" => {
                for row in y1..=y2 {
                    for col in x1..=x2 {
                        lights[row][col] = false;
                    }
                }
            }
            _ => unreachable!(),
        };
    }
    let mut enabled_lights = 0;
    for row in lights {
        for light in row {
            if light {
                enabled_lights += 1;
            }
        }
    }
    Ok(enabled_lights)
}

pub fn solve_day_06_part_2(input: &str) -> Result<u64, String> {
    let re = Regex::new(r"(turn on|toggle|turn off) (\d+),(\d+) through (\d+),(\d+)").unwrap();
    let mut lights = vec![vec![0; 1000]; 1000];
    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        let (action, x1, y1, x2, y2) = (
            &caps[1],
            caps[2].parse::<usize>().unwrap(),
            caps[3].parse::<usize>().unwrap(),
            caps[4].parse::<usize>().unwrap(),
            caps[5].parse::<usize>().unwrap(),
        );
        // I think it's more clear to explicitly index into lights rather than
        // construct an iterator over it.
        #[allow(clippy::needless_range_loop)]
        match action {
            "turn on" => {
                for row in y1..=y2 {
                    for col in x1..=x2 {
                        lights[row][col] += 1;
                    }
                }
            }
            "toggle" => {
                for row in y1..=y2 {
                    for col in x1..=x2 {
                        lights[row][col] += 2;
                    }
                }
            }
            "turn off" => {
                for row in y1..=y2 {
                    for col in x1..=x2 {
                        if lights[row][col] > 0 {
                            lights[row][col] -= 1;
                        };
                    }
                }
            }
            _ => unreachable!(),
        };
    }
    let mut total_brightness = 0;
    for row in lights {
        for brightness in row {
            total_brightness += brightness
        }
    }
    Ok(total_brightness)
}

pub fn solve_day_10_part_1(input: &str) -> Result<u64, String> {
    let mut sequence = input.to_string();
    for _ in 0..40 {
        let mut new_sequence = Vec::new();
        let mut digit = sequence.chars().next().unwrap();
        let mut run = 0;
        for chr in sequence.chars() {
            if digit == chr {
                run += 1;
            } else {
                new_sequence.push(run.to_string());
                new_sequence.push(digit.to_string());
                digit = chr;
                run = 1;
            }
        }
        new_sequence.push(run.to_string());
        new_sequence.push(digit.to_string());
        sequence = new_sequence.iter().fold(String::new(), |acc, s| acc + s);
    }
    Ok(sequence.len() as u64)
}

pub fn solve_day_12_part_1(input: &str) -> Result<u64, String> {
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

pub fn solve_day_25_part_1(input: &str) -> Result<u64, String> {
    let re = Regex::new(r"row (\d+), column (\d+)").unwrap();
    let caps = re.captures(input).unwrap();
    let (row, column) = (
        caps[1].parse::<i64>().unwrap(),
        caps[2].parse::<i64>().unwrap(),
    );
    let mut num_iterations = ((row + column - 2).pow(2) + (row + column - 2) + 2) / 2 + column - 2;
    // The pattern repeats, so skip as many complete cycles as we can.
    while num_iterations > 16_777_196 {
        num_iterations -= 16_777_196;
    }

    let mut code = 20_151_125;
    let m = 252_533;
    let n = 33_554_393;
    for _ in 0..num_iterations {
        code = (code * m) % n;
    }

    Ok(code)
}

#[cfg(test)]
mod test {
    use super::*;
    use util::check_solution;

    #[test]
    fn day_01_part_1() {
        check_solution("(())\n", 0, &solve_day_01_part_1);
        check_solution("()()\n", 0, &solve_day_01_part_1);
        check_solution("(((\n", 3, &solve_day_01_part_1);
        check_solution("(()(()(\n", 3, &solve_day_01_part_1);
        check_solution("))(((((\n", 3, &solve_day_01_part_1);
    }

    #[test]
    fn day_01_part_2() {
        check_solution(")\n", 1, &solve_day_01_part_2);
        check_solution("()())\n", 5, &solve_day_01_part_2);
    }

    const DAY_02_SAMPLE_INPUT: &str = "2x3x4\n1x1x10\n";

    #[test]
    fn day_02_part_1() {
        check_solution(&DAY_02_SAMPLE_INPUT, 101, &solve_day_02_part_1);
    }

    #[test]
    fn day_02_part_2() {
        check_solution(&DAY_02_SAMPLE_INPUT, 48, &solve_day_02_part_2);
    }

    #[test]
    fn day_03_part_1() {
        check_solution(">", 2, &solve_day_03_part_1);
        check_solution("^>v<", 4, &solve_day_03_part_1);
        check_solution("^v^v^v^v^v", 2, &solve_day_03_part_1);
    }

    // #[test]
    // fn day_03_part_2() {
    //     // check_solution(input, output, solution);
    //     unimplemented!();
    // }

    #[test]
    fn test_md5_short() {
        assert_eq!(md5_short(b""), 0xd41d8cd98f00b204e9800998ecf8427e);
        assert_eq!(md5_short(b"a"), 0x0cc175b9c0f1b6a831c399e269772661);
        assert_eq!(md5_short(b"abc"), 0x900150983cd24fb0d6963f7d28e17f72);
        assert_eq!(
            md5_short(b"message digest"),
            0xf96b697d7cb7938d525a2f31aaf161d0
        );
        assert_eq!(
            md5_short(b"abcdefghijklmnopqrstuvwxyz"),
            0xc3fcd3d76192e4007dfb496cca67e13b
        );
    }

    #[test]
    #[ignore]
    fn day_04_part_1() {
        check_solution("abcdef", 609043, &solve_day_04_part_1);
        check_solution("pqrstuv", 1048970, &solve_day_04_part_1);
    }

    #[test]
    fn day_05_part_1() {
        check_solution("ugknbfddgicrmopn\n", 1, &solve_day_05_part_1);
        check_solution("aaa\n", 1, &solve_day_05_part_1);
        check_solution("jchzalrnumimnmhp\n", 0, &solve_day_05_part_1);
        check_solution("haegwjzuvuyypxyu\n", 0, &solve_day_05_part_1);
        check_solution("dvszwmarrgswjxmb\n", 0, &solve_day_05_part_1);
    }

    #[test]
    fn day_06_part_1() {
        check_solution(
            "turn on 0,0 through 999,999\n",
            1_000_000,
            &solve_day_06_part_1,
        );
        check_solution(
            "turn on 0,0 through 999,999\ntoggle 0,0 through 999,0",
            999_000,
            &solve_day_06_part_1,
        );
        check_solution(
            "turn on 0,0 through 999,999\nturn off 499,499 through 500,500",
            999_996,
            &solve_day_06_part_1,
        );
    }

    #[test]
    fn day_06_part_2() {
        check_solution("turn on 0,0 through 0,0", 1, &solve_day_06_part_2);
        check_solution(
            "toggle 0,0 through 999,999",
            2_000_000,
            &solve_day_06_part_2,
        );
    }

    #[test]
    fn day_10_part_1() {
        check_solution("1", 82_350, &solve_day_10_part_1);
    }

    #[test]
    fn day_12_part_1() {
        check_solution("[1,2,3]", 6, &solve_day_12_part_1);
        check_solution("{\"a\":2,\"b\":4}", 6, &solve_day_12_part_1);
        check_solution("[[[3]]]", 3, &solve_day_12_part_1);
        check_solution("{\"a\":{\"b\":4},\"c\":-1}", 3, &solve_day_12_part_1);
        check_solution("{\"a\":[-1,1]}", 0, &solve_day_12_part_1);
        check_solution("[-1,{\"a\":1}]", 0, &solve_day_12_part_1);
        check_solution("[]", 0, &solve_day_12_part_1);
        check_solution("{}", 0, &solve_day_12_part_1);

        // Extra multi-digit test case
        check_solution("{\"a\":1854,\"b\":-430}", 1_424, &solve_day_12_part_1);
    }

    #[test]
    fn day_25_part_1() {
        check_solution("row 6, column 6", 27995004, &solve_day_25_part_1);
    }
}
