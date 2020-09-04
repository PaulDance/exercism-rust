pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut is_prime = Vec::<bool>::new();
    is_prime.resize(upper_bound as usize - 1, true);

    for i in 0..is_prime.len() {
        if is_prime[i] {
            let mut factor = 2;
            let mut mult = factor * (i + 2) - 2;

            while mult < is_prime.len() {
                is_prime[mult] = false;
                factor += 1;
                mult = factor * (i + 2) - 2;
            }
        }
    }

    is_prime
        .iter()
        .enumerate()
        .filter_map(|(i, &p)| if p { Some(i as u64 + 2) } else { None })
        .collect()
}
