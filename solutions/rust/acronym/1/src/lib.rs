pub fn abbreviate(phrase: &str) -> String {
    phrase
        .replace('-', " ")
        .split_whitespace()
        .flat_map(|word| {
            let mut letters = Vec::new();

            let w = word.chars().filter(|c| c.is_alphabetic()).collect::<Vec<_>>();

            if let Some(first) = w.iter().next() {
                letters.push(first.to_ascii_uppercase());
            }

            if w.len() <= 1 || w.iter().all(|c| c.is_ascii_uppercase()) {
                return letters;
            }

            letters.extend(w.iter().skip(1).filter(|c| c.is_ascii_uppercase()));

            letters
        })
        .collect()
}