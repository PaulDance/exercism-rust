/// Solves the problem using iterators.
pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    input
        .iter()
        .enumerate()
        .flat_map(|(i, line)| {
            line.iter()
                .enumerate()
                .filter(move |&(j, _)| is_saddle_point(i, j, input))
                .map(move |(j, _)| (i, j))
        })
        .collect()
}

/// Returns true iff `matrix[line][col]` is a saddle point per the exercise's definition.
fn is_saddle_point(line: usize, col: usize, matrix: &[Vec<u64>]) -> bool {
    // Return false as soon as the column rule reveals unverified.
    for i in 0..matrix.len() {
        if matrix[i][col] < matrix[line][col] {
            return false;
        }
    }

    // Return false as soon as the line rule reveals unverified.
    for j in 0..matrix[0].len() {
        if matrix[line][j] > matrix[line][col] {
            return false;
        }
    }

    true
}
