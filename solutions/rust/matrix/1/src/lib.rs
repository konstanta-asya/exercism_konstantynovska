pub struct Matrix {
    data: Vec<Vec<u32>>,
}

impl Matrix {
    pub fn new(input: &str) -> Self {
        let data = input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|num| num.parse::<u32>().expect("Invalid number in matrix"))
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<Vec<u32>>>();

        Matrix { data }
    }

    pub fn row(&self, row_no: usize) -> Option<Vec<u32>> {
        self.data.get(row_no.checked_sub(1)?).cloned()
    }

    pub fn column(&self, col_no: usize) -> Option<Vec<u32>> {
        let col_index = col_no.checked_sub(1)?;
        if self.data.first()?.len() <= col_index {
            return None;
        }

        Some(self.data.iter().map(|row| row[col_index]).collect())
    }
}
