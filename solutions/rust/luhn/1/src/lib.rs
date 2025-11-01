pub fn is_valid(code: &str) -> bool {
    if code.chars().any(|c| !c.is_ascii_digit() && c != ' ') {
        return false;
    }
    
    let digits: Vec<u32> = code
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect();

    if digits.len() <= 1 {
        return false;
    }

    let sum: u32 = digits
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &d)| {
            if i % 2 == 1 {
                let doubled = d * 2;
                if doubled > 9 { doubled - 9 } else { doubled }
            } else {
                d
            }
        })
        .sum();

    sum % 10 == 0
}
