use crate::util::{parse_u64, AdventResult};

fn transform_subject_number(subject_number: u64, loop_size: u64) -> u64 {
    let mut result = 1;
    for _ in 1..=loop_size {
        result = (result * subject_number) % 20201227;
    }
    result
}

pub fn part_1(input: &str) -> AdventResult {
    let public_keys = parse_u64(input);
    let (card_pubkey, door_pubkey) = (public_keys[0], public_keys[1]);

    // try lots of loop sizes
    // stop when we've found both pubkeys somewhere in the output
    let mut test_loop_size = 1;
    let (mut card_loop_size, mut door_loop_size): (Option<u64>, Option<u64>) = (None, None);
    let mut test_pubkey = 1;
    while card_loop_size.is_none() || door_loop_size.is_none() {
        test_pubkey = (test_pubkey * 7) % 20201227;
        if test_pubkey == card_pubkey {
            // we found the card's loop size
            card_loop_size = Some(test_loop_size);
        }
        if test_pubkey == door_pubkey {
            // we found the door's loop size
            door_loop_size = Some(test_loop_size);
        }
        test_loop_size += 1;
    }
    // use the loop sizes to compute the encryption key
    let encryption_key = transform_subject_number(card_pubkey, door_loop_size.unwrap());
    // there are two ways to compute the encryption key, and they must match
    assert_eq!(
        transform_subject_number(door_pubkey, card_loop_size.unwrap()),
        encryption_key
    );
    Ok(encryption_key)
}

#[cfg(test)]
use crate::util::check_solution;

#[test]
fn test_part_1() {
    assert_eq!(transform_subject_number(7, 8), 5764801);
    assert_eq!(transform_subject_number(7, 11), 17807724);
    assert_eq!(transform_subject_number(17807724, 8), 14897079);
    assert_eq!(transform_subject_number(5764801, 11), 14897079);
    check_solution("5764801\n17807724\n", 14897079, &part_1);
}
