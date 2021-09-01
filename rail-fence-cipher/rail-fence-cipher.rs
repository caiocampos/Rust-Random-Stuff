pub struct RailFence {
    rails: u32,
}

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        RailFence { rails: rails }
    }

    pub fn encode(&self, text: &str) -> String {
        let vec: Vec<char> = text.chars().collect();
        let fence = self.fence(vec.len());
        (0..vec.len()).map(|i| vec[fence[i]]).collect()
    }

    pub fn decode(&self, cipher: &str) -> String {
        let vec: Vec<char> = cipher.chars().collect();
        let fence = self.fence(vec.len());
        (0..vec.len())
            .map(|i| vec[fence.iter().position(|&r| r == i).unwrap()])
            .collect()
    }

    fn fence(&self, size: usize) -> Vec<usize> {
        let mut fence = new_matrix(size as u32, self.rails);
        let mut rails = (0..self.rails).chain((1..self.rails - 1).rev()).cycle();
        (0..size).for_each(|i| fence[rails.next().unwrap() as usize][i] = i as i32);
        (0..self.rails as usize)
            .flat_map(|i| {
                (0..size)
                    .filter_map(|j| {
                        if fence[i][j] >= 0 {
                            Some(fence[i][j] as usize)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<usize>>()
            })
            .collect()
    }
}

fn new_matrix(x: u32, y: u32) -> Vec<Vec<i32>> {
    let v: Vec<i32> = (0..x).map(|_| -1).collect();
    (0..y).map(|_| v.clone()).collect()
}
