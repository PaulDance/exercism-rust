const GRID_SIZE: u32 = 8 * 8;

pub fn square(s: u32) -> u64 {
    if s == 0 || s > GRID_SIZE {
        panic!("Square must be between 1 and {}.", GRID_SIZE)
    } else {
        2u64.pow(s - 1)
    }
}

pub fn total() -> u64 {
    // Naive solution: (1..=GRID_SIZE).map(square).sum()
    // "Direct" solution:
    (2u128.pow(GRID_SIZE + 1) - 1) as u64
}
