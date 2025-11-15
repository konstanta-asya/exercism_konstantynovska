pub fn egg_count(display_value: u32) -> usize {
    let mut num = display_value;
    let mut count = 0;

    while num > 0 {
        if num % 2 == 1 {
            count += 1;
        }
        num = num / 2;
    }
    count
}