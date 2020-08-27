pub fn factors(n: u64) -> Vec<u64> {
    let mut x = n;
    let mut factors = Vec::<u64>::new();

    for prime in (2..=(n as f64).sqrt() as u64 + 1).filter(|&p| (2..p).all(|q| p % q != 0)) {
        while x % prime == 0 {
            factors.push(prime);
            x /= prime;
        }
    }

    factors
}
