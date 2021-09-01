use rayon::prelude::*;

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let cols = invert_matrix(input);
    let (max_vals, min_vals) = rayon::join(|| max_list(input), || min_list(&cols));
    input
        .par_iter()
        .enumerate()
        .flat_map(|(i, vec)| {
            vec.par_iter()
                .enumerate()
                .filter_map(|(j, el)| {
                    if el == max_vals[i] && el == min_vals[j] {
                        Some((i, j))
                    } else {
                        None
                    }
                })
                .collect::<Vec<(usize, usize)>>()
        })
        .collect()
}

/* Inverts the matrix
| 1 2 3 |    | 1 4 7 |
| 4 5 6 | => | 2 4 8 |
| 7 8 9 |    | 3 6 9 |
*/
pub fn invert_matrix(input: &[Vec<u64>]) -> Vec<Vec<u64>> {
    if input.len() > 0 {
        (0..input[0].len())
            .map(|i| (0..input.len()).map(|j| input[j][i]).collect())
            .collect()
    } else {
        Vec::new()
    }
}

/* Finds the minimal values
| 1 2 3 |
| 4 5 6 | => [1 4 7]
| 7 8 9 |
*/
pub fn min_list(input: &[Vec<u64>]) -> Vec<&u64> {
    input
        .iter()
        .filter_map(|v| v.iter().min())
        .collect()
}

/* Finds the maximal values
| 1 2 3 |
| 4 5 6 | => [3 6 9]
| 7 8 9 |
*/
pub fn max_list(input: &[Vec<u64>]) -> Vec<&u64> {
    input
        .iter()
        .filter_map(|v| v.iter().max())
        .collect()
}
