pub fn is_armstrong_number(num: u64) -> bool {
    let num_str = num.to_string();
    num_str
        .chars()
        .map(|chr| (chr.to_digit(10).unwrap() as u64).pow(num_str.len() as u32))
        .sum::<u64>()
        == num
}
