pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let n = size as usize;
    let mut matrix = vec![vec![0; n]; n];
    let mut num = 1;
    let mut top = 0;
    let mut bottom = n as isize - 1;
    let mut left = 0;
    let mut right = n as isize - 1;

    while top <= bottom && left <= right {
        for j in left..=right {
            matrix[top as usize][j as usize] = num;
            num += 1;
        }
        top += 1;

        for i in top..=bottom {
            matrix[i as usize][right as usize] = num;
            num += 1;
        }
        right -= 1;

        if top <= bottom {
            for j in (left..=right).rev() {
                matrix[bottom as usize][j as usize] = num;
                num += 1;
            }
            bottom -= 1;
        }

        if left <= right {
            for i in (top..=bottom).rev() {
                matrix[i as usize][left as usize] = num;
                num += 1;
            }
            left += 1;
        }
    }

    matrix
}
