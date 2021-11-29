pub fn parse_u64(input: &String) -> Vec<u64> {
    input
        .split_terminator('\n')
        .map(|x| x.parse::<u64>().unwrap())
        .collect()
}
