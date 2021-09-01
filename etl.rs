use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h.iter()
        .flat_map(|(&val, list)| {
            list.iter()
                .map(|&c| (c.to_ascii_lowercase(), val))
                .collect::<Vec<(char, i32)>>()
        })
        .collect()
}
