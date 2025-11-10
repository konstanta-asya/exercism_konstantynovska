pub fn number(user_number: &str) -> Option<String> {

    let digits: String = user_number.chars().filter(|c| c.is_ascii_digit()).collect();

    let cleaned = if digits.len() == 11 && digits.starts_with('1') {
        digits[1..].to_string()
    } else if digits.len() == 10 {
        digits
    } else {
        return None;
    };

    let chars: Vec<char> = cleaned.chars().collect();
    let area_first = chars[0];
    let exchange_first = chars[3];

    if area_first < '2' || exchange_first < '2' {
        return None;
    }

    Some(cleaned)
}