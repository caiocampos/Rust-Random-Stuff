pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let matrix = to_matrix(minefield);
    matrix
        .iter()
        .enumerate()
        .map(|(i, line)| {
            (0..line.len())
                .map(|j| get_annotation(&matrix, i, j))
                .collect()
        })
        .collect()
}

fn to_matrix(minefield: &[&str]) -> Vec<Vec<char>> {
    minefield
        .iter()
        .map(|line| line.chars().collect())
        .collect()
}

const ZERO: u8 = '0' as u8;

fn get_annotation(minefield: &Vec<Vec<char>>, i: usize, j: usize) -> char {
    match minefield[i][j] {
        ' ' => {
            let (v_min, h_min) = (get_min(i), get_min(j));
            let v_max = get_max(i, minefield.len());
            let h_max = get_max(j, minefield[i].len());
            let count = (v_min..v_max).fold(0usize, |sum, m| {
                sum + (h_min..h_max).filter(|&n| minefield[m][n] == '*').count()
            });
            match count {
                0 => ' ',
                x => (ZERO + x as u8) as char,
            }
        }
        c => c,
    }
}

fn get_min(n: usize) -> usize {
    match n {
        0 => 0,
        x => x - 1,
    }
}

fn get_max(n: usize, max: usize) -> usize {
    (max).min(n + 2)
}
