pub fn square(s: u32) -> u64 {
    if (s > 64) { panic!(); }
    1u64.wrapping_shl(s - 1)
}

pub fn total() -> u64 {
    u64::MAX
}
