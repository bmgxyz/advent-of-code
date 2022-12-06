use std::num::Wrapping;

use crate::util::{AdventResult, AdventSolution};

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

pub fn part_1(input: &str) -> AdventResult {
    let mut number = 0;
    let original_message = input.as_bytes();
    // Run until the first 20 bits are zero.
    while md5_short(&concat_message(original_message, number)) >> 108 != 0 {
        number += 1;
    }
    Ok(AdventSolution::from(number))
}

pub fn part_2(input: &str) -> AdventResult {
    let mut number = 0;
    let original_message = input.as_bytes();
    // Run until the first 24 bits are zero.
    while md5_short(&concat_message(original_message, number)) >> 104 != 0 {
        number += 1;
    }
    Ok(AdventSolution::from(number))
}

#[cfg(test)]
use crate::util::check_solution;

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
fn test_part_1() {
    check_solution("abcdef", 609043, &part_1);
    check_solution("pqrstuv", 1048970, &part_1);
}

#[test]
#[ignore]
fn test_part_2() {
    check_solution("abcdef", 6742839, &part_2);
    check_solution("pqrstuv", 5714438, &part_2);
}
