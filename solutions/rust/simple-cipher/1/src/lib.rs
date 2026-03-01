use std::time::{SystemTime, UNIX_EPOCH};

fn shift(c: char, key_shift: i8, encode: bool) -> char {
    let base = b'a';
    let offset = c as i8 - base as i8;
    let shifted = if encode {
        (offset + key_shift).rem_euclid(26)
    } else {
        (offset - key_shift).rem_euclid(26)
    };
    (base + shifted as u8) as char
}

pub fn encode(key: &str, s: &str) -> Option<String> {
    if key.is_empty() || !key.chars().all(|c| c.is_ascii_lowercase()) {
        return None;
    }

    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if !c.is_ascii_lowercase() {
            return None;
        }
        let key_char = key.chars().nth(i % key.len()).unwrap();
        let key_shift = (key_char as u8 - b'a') as i8;
        result.push(shift(c, key_shift, true));
    }
    Some(result)
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    if key.is_empty() || !key.chars().all(|c| c.is_ascii_lowercase()) {
        return None;
    }

    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if !c.is_ascii_lowercase() {
            return None;
        }
        let key_char = key.chars().nth(i % key.len()).unwrap();
        let key_shift = (key_char as u8 - b'a') as i8;
        result.push(shift(c, key_shift, false));
    }
    Some(result)
}

pub fn encode_random(s: &str) -> (String, String) {

    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();

    let mut seed = nanos as u64;
    let mut key = String::new();

    for _ in 0..100 {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        let ch = ((seed % 26) as u8 + b'a') as char;
        key.push(ch);
    }

    let encoded = encode(&key, s).unwrap_or_default();
    (key, encoded)
}
