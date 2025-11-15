use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut result = BTreeMap::new();

    for (score, letters) in h.iter() {
        for letter in letters {
            let lower_case_letter = letter.to_ascii_lowercase();
            result.insert(lower_case_letter, *score);
        }
    }
    result
}