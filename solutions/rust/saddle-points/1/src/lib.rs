pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    if input.is_empty() || input[0].is_empty() {
        return vec![];
    }

    let rows = input.len();
    let cols = input[0].len();

    let row_maxes: Vec<u64> = input.iter()
        .map(|row| *row.iter().max().unwrap())
        .collect();

    let mut col_mins: Vec<u64> = vec![u64::MAX; cols];
    for c in 0..cols {
        for r in 0..rows {
            if input[r][c] < col_mins[c] {
                col_mins[c] = input[r][c];
            }
        }
    }

    let mut saddle_points = vec![];
    for r in 0..rows {
        for c in 0..cols {
            if input[r][c] == row_maxes[r] && input[r][c] == col_mins[c] {
                saddle_points.push((r, c));
            }
        }
    }

    saddle_points
}
