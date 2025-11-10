pub fn actions(n: u8) -> Vec<&'static str> {
    let mut res = Vec::new();

    if n & 0b00001 != 0 {
        res.push("wink");
    }
    if n & 0b00010 != 0 {
        res.push("double blink");
    }
    if n & 0b00100 != 0 {
        res.push("close your eyes");
    }
    if n & 0b01000 != 0 {
        res.push("jump");
    }

    if n & 0b10000 != 0 {
        res.reverse();
    }

    res
}
