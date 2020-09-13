pub fn spiral_matrix(size: usize) -> Vec<Vec<u32>> {
    if size == 0 {
        Vec::new()
    } else {
        // Position and counter.
        let mut i = 0;
        let mut j = 0;
        let mut counter = 1;

        // Grid initialization: early allocation and filled with zeros.
        let mut grid = Vec::with_capacity(size);
        grid.extend((0..size).map(|_| {
            let mut line = Vec::with_capacity(size);
            line.extend((0..size).map(|_| 0));
            line
        }));

        // Main loop: we detect visited nodes with non-zero values.
        while grid[i][j] == 0 {
            // Top left to top right.
            while j < size && grid[i][j] == 0 {
                grid[i][j] = counter;
                counter += 1;
                j += 1;
            }

            // Cancel last increment of j and go to next line.
            j = j.saturating_sub(1);
            i = i.saturating_add(1).min(size - 1);

            // Top right to bottom right.
            while i < size && grid[i][j] == 0 {
                grid[i][j] = counter;
                counter += 1;
                i += 1;
            }

            // Cancel last increment of i and go to previous column.
            i = i.saturating_sub(1);
            j = j.saturating_sub(1);
            let mut border_reached = false;

            // Bottom right to bottom left.
            while grid[i][j] == 0 {
                grid[i][j] = counter;
                counter += 1;

                if j == 0 {
                    border_reached = true;
                    break;
                } else {
                    j -= 1;
                }
            }

            // Cancel last decrement of j only when not having reached the border,
            if !border_reached {
                j = j.saturating_add(1).min(size - 1);
            }

            // and go to previous line.
            i = i.saturating_sub(1);

            // Bottom left to top left.
            while i != 0 && grid[i][j] == 0 {
                grid[i][j] = counter;
                counter += 1;
                i -= 1;
            }

            // Cancel last decrement of i (border is never reached) and go to next column.
            i = i.saturating_add(1).min(size - 1);
            j = j.saturating_add(1).min(size - 1);
        }

        grid
    }
}
