#[derive(Debug)]
pub struct Item {
    pub weight: u32,
    pub value: u32,
}

pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
    let n = items.len();
    let mut dp = vec![0u32; (max_weight + 1) as usize];

    for item in items {
        for w in (item.weight..=max_weight).rev() {
            let idx = w as usize;
            let prev_idx = (w - item.weight) as usize;
            dp[idx] = dp[idx].max(dp[prev_idx] + item.value);
        }
    }

    dp[max_weight as usize]
}
