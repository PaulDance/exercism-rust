/// A 2-D, `u32`-backed, 1-indexed matrix.
pub struct Matrix {
    // Not using a more performant single array for simplicity.
    cells: Vec<Vec<u32>>,
}

impl Matrix {
    /// Parses the `input` string into a [`Matrix`].
    pub fn new(input: &str) -> Self {
        Self {
            cells: input
                .lines()
                .map(|line| {
                    line.split_whitespace()
                        .map(|num| num.parse().unwrap())
                        .collect()
                })
                .collect(),
        }
    }

    /// Returns the row at the 1-based `row` index or `None` if not present.
    pub fn row(&self, row: usize) -> Option<Vec<u32>> {
        self.cells.get(row.checked_sub(1)?).cloned()
    }

    /// Returns the column at the 1-based `col` index or `None` if not present.
    pub fn column(&self, col: usize) -> Option<Vec<u32>> {
        self.cells
            .iter()
            .map(|row| row.get(col.checked_sub(1)?).copied())
            .collect()
    }
}
