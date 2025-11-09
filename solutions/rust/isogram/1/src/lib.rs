use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut seen = HashSet::new();

    for c in candidate.chars() {
        let c = c.to_ascii_lowercase();
        if c.is_alphabetic() {
            if !seen.insert(c) {
                return false;
            }
        }
    }

    true
}
