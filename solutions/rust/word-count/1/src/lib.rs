use std::collections::HashMap;

pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut counts = HashMap::new();
    let mut current_word = String::new();

    for c in words.chars() {
        if c.is_ascii_alphanumeric() || c == '\'' {
            current_word.push(c.to_ascii_lowercase());
        } else {
            if !current_word.is_empty() {
                let trimmed = current_word.trim_matches('\'').trim_matches('"').to_string();
                if !trimmed.is_empty() {
                    *counts.entry(trimmed).or_insert(0) += 1;
                }
                current_word.clear();
            }
        }
    }

    if !current_word.is_empty() {
        let trimmed = current_word.trim_matches('\'').trim_matches('"').to_string();
        if !trimmed.is_empty() {
            *counts.entry(trimmed).or_insert(0) += 1;
        }
    }

    counts
}
