pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: usize) -> Self {
        let mut rows = Vec::<Vec<u32>>::with_capacity(row_count);

        for i in 0..row_count {
            rows.push(Vec::with_capacity(i + 1));
            rows[i].push(1);

            if i > 0 {
                for j in 1..i {
                    let x = rows[i - 1][j - 1] + rows[i - 1][j];
                    rows[i].push(x);
                }

                rows[i].push(1);
            }
        }

        Self { rows }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}
