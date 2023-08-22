// For simpler performant dense array access.
use ndarray::Array2;

/// Bag item information.
pub struct Item {
    pub weight: u32,
    pub value: u32,
}

/// Solves the knapsack problem for the given `items` and `max_weight`.
///
/// Dynamic implementation taken from [Wikipedia].
///
/// [Wikipedia]: https://en.wikipedia.org/wiki/Knapsack_problem#0-1_knapsack_problem
pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
    // Handle empty case separately in order to avoid useless allocation.
    if items.is_empty() {
        0
    } else {
        // Only the last row is required in order to compute the next one.
        let mut mat = Array2::<u32>::zeros((2, max_weight as usize + 1));
        // Current row index. Its 1-complement is the next row.
        let mut row = 0;

        for i in 0..items.len() {
            for j in 1..=max_weight as usize {
                mat[(1 - row, j)] = if items[i].weight as usize > j {
                    mat[(row, j)]
                } else {
                    mat[(row, j)].max(mat[(row, j - items[i].weight as usize)] + items[i].value)
                }
            }

            // This effectively "swaps" the two rows as all row indexing
            // operations in the next loop iteration will be switched compared
            // to the current one.
            row = 1 - row;
        }

        // Here `row` has just been switched, so is already the last row index.
        mat[(row, max_weight as usize)]
    }
}
