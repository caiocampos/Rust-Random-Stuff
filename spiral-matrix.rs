pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    match size {
        0 => Vec::new(),
        1 => vec![vec![1]],
        _ => {
            let max = size * size;
            let mut m = new_matrix(size);
            let mut update = |i: usize, j: usize, v: u32| {
                m[i][j] = v;
                v + 1
            };
            let (mut val, mut b, mut e) = (1, 0 as usize, size as usize - 1);
            while val <= max {
                (b..=e).for_each(|i| val = update(b, i, val));

                // ((b + 1)..=e).for_each(|i| val = update(i, e, val));
                (b..=e).skip(1).for_each(|i| val = update(i, e, val));

                (b..e).rev().for_each(|i| val = update(e, i, val));

                // ((b + 1)..e).rev().for_each(|i| val = update(i, b, val));
                (b..e).skip(1).rev().for_each(|i| val = update(i, b, val));
                b += 1;
                e -= 1;
            }
            m
        }
    }
}

fn new_matrix(size: u32) -> Vec<Vec<u32>> {
    let v: Vec<u32> = (0..size).map(|_| 0).collect();
    (0..size).map(|_| v.clone()).collect()
}
