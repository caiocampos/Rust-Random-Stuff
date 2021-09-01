pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle {
            rows: fill(row_count),
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}

fn fill(row_count: u32) -> Vec<Vec<u32>> {
    (0..row_count as usize).fold(Vec::new(), |mut rows, i| {
        let get_value = |j| {
            if j == 0 || j == i {
                1
            } else {
                rows[i - 1][j] + rows[i - 1][j - 1]
            }
        };
        let row: Vec<u32> = (0..=i).map(get_value).collect();
        rows.push(row);
        rows
    })
}
