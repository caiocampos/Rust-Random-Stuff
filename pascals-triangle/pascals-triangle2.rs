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
    let mut m = Math::new();
    (0..row_count)
        .map(|i| (0..=i).map(|j| m.binomial_coefficient(i, j)).collect())
        .collect()
}

pub struct Math<T> {
    fat_buf: Vec<T>,
}

impl Math<u32> {
    pub fn new() -> Self {
        Math {
            fat_buf: vec![1, 1, 2],
        }
    }

    fn binomial_coefficient(&mut self, n: u32, k: u32) -> u32 {
        let nf = self.factorial(n);
        let kf = self.factorial(k);
        nf / (kf * self.factorial(n - k))
    }

    fn factorial(&mut self, n: u32) -> u32 {
        let min = self.fat_buf.len() as u32;
        if n > min - 1 {
            (min..=n).for_each(|i| {
                let val = i * self.fat_buf[i as usize - 1];
                self.fat_buf.push(val);
            });
        }
        self.fat_buf[n as usize]
    }
}
