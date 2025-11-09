pub struct PascalsTriangle {
    triangle: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut triangle: Vec<Vec<u32>> = Vec::new();

        for i in 0..row_count {
            let mut row = Vec::with_capacity((i + 1) as usize);
            for j in 0..=i {
                if j == 0 || j == i {
                    row.push(1);
                } else {
                    let left = triangle[i as usize - 1][j as usize - 1];
                    let right = triangle[i as usize - 1][j as usize];
                    row.push(left + right);
                }
            }
            triangle.push(row);
        }

        PascalsTriangle { triangle }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.triangle.clone()
    }
}
