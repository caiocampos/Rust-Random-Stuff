pub struct RailFence {
    rails: usize
}

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        RailFence { rails: rails as usize }
    }

    pub fn encode(&self, text: &str) -> String {
        let mut result = vec![Vec::new(); self.rails];
        for (c, i) in text.chars().zip(zigzag(self.rails)) {
            result[i].push(c);
        }
        result.iter().flat_map(|c| c).collect::<String>()
    }

    pub fn decode(&self, cipher: &str) -> String {
        let mut indexes : Vec<_> = zigzag(self.rails).zip(1..).take(cipher.len()).collect();
        indexes.sort();

        let mut char_with_index : Vec<_> = cipher.chars().zip(indexes).map(|(c, (_, i))| (i, c)).collect();
        char_with_index.sort();
        char_with_index.iter().map(|(_, c)| c).collect()
    }
}

fn zigzag(n: usize) -> impl Iterator<Item = usize> {
    (0..n - 1).chain((1..n).rev()).cycle()
}