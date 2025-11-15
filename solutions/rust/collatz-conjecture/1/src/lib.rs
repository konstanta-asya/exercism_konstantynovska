pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    let mut num = n;
    let mut count = 0;
    while num != 1 {
        if num % 2 == 0 {
            num = num / 2;
        } else {
            let next_val = num.checked_mul(3)?.checked_add(1)?;
            num = next_val;
        }
        count += 1;
    }
    Some(count)
}