/// Returns the count of non-zero bits in the given integer `value`.
pub fn egg_count(value: u32) -> usize {
    (0..u32::BITS).map(|i| (value & (1 << i)) >> i).sum::<u32>() as _
}
