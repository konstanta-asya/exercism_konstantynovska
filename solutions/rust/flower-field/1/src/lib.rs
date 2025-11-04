pub fn annotate(garden: &[&str]) -> Vec<String> {
    if garden.is_empty() {
        return vec![];
    }

    let height = garden.len();
    let width = garden[0].len();
    let mut result = vec![vec![0u8; width]; height];

    for (y, row) in garden.iter().enumerate() {
        for (x, &tile) in row.as_bytes().iter().enumerate() {
            if tile != b'*' {
                continue;
            }

            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dy == 0 && dx == 0 {
                        continue;
                    }

                    let ny = y as isize + dy;
                    let nx = x as isize + dx;

                    if ny >= 0 && ny < height as isize && nx >= 0 && nx < width as isize {
                        result[ny as usize][nx as usize] += 1;
                    }
                }
            }
        }
    }

    result
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, &count)| {
                    if garden[y].as_bytes()[x] == b'*' {
                        '*'
                    } else if count == 0 {
                        ' '
                    } else {
                        (b'0' + count) as char
                    }
                })
                .collect()
        })
        .collect()
}
