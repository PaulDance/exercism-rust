/// Returns the number of steps required to verify the Collatz conjecture for `n`
/// if it is not equal to zero, `None` otherwise.
pub fn collatz(mut n: u64) -> Option<u64> {
    if n == 0 {
        None
    } else {
        let mut steps = 0;

        while n != 1 {
            if n % 2 == 0 {
                n /= 2;
            } else {
                n = 3 * n + 1;
            }

            steps += 1;
        }

        Some(steps)
    }
}
