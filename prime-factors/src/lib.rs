pub fn factors(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::<u64>::new();
    let mut k = 2;

    while n > 1 {
        while n % k == 0 {
            factors.push(k);
            n /= k;
        }

        k += 1;
    }

    factors
}
