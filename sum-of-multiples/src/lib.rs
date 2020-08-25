pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .filter(|n| {
            factors.iter().any(|p| match p {
                0 => false,
                _ => n % p == 0,
            })
        })
        .sum()
}
