pub fn is_valid_isbn(isbn: &str) -> bool {

    let cleaned: String = isbn.chars().filter(|c| *c != '-').collect();

    if cleaned.len() != 10 {
        return false;
    }

    let mut sum = 0;
    for (i, ch) in cleaned.chars().enumerate() {
        let value = match ch {
            '0'..='9' => ch.to_digit(10).unwrap(),
            'X' if i == 9 => 10, 
            _ => return false,   
        };

        sum += value * (10 - i as u32);
    }

    sum % 11 == 0
}
