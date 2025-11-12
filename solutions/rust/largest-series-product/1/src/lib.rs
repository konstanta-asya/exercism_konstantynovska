#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span > string_digits.len() {
        return Err(Error::SpanTooLong);
    }
    if span == 0 {
        return Ok(1);
    }

    let digits: Result<Vec<u64>, Error> = string_digits
        .chars()
        .map(|c| c.to_digit(10)
            .map(|d| d as u64)
            .ok_or(Error::InvalidDigit(c)))
        .collect();
    let digits = digits?;

    let mut max_product = 0;
    for window in digits.windows(span) {
        let product = window.iter().product();
        if product > max_product {
            max_product = product;
        }
    }

    Ok(max_product)
}
