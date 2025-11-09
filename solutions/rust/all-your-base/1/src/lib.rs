#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    }
    if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }

    let mut decimal: u128 = 0;
    for &digit in number {
        if digit >= from_base {
            return Err(Error::InvalidDigit(digit));
        }
        decimal = decimal
            .checked_mul(from_base as u128)
            .and_then(|v| v.checked_add(digit as u128))
            .ok_or(Error::InvalidDigit(digit))?;
    }

    if decimal == 0 {
        return Ok(vec![0]);
    }

    let mut result = Vec::new();
    let mut n = decimal;
    while n > 0 {
        result.push((n % to_base as u128) as u32);
        n /= to_base as u128;
    }

    result.reverse();
    Ok(result)
}
