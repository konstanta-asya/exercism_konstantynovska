#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let tmp = b;
        b = a % b;
        a = tmp;
    }
    a
}

fn mod_inv(a: i32, m: i32) -> Option<i32> {
    let mut t = 0;
    let mut new_t = 1;
    let mut r = m;
    let mut new_r = a;

    while new_r != 0 {
        let quotient = r / new_r;
        let tmp_t = t - quotient * new_t;
        t = new_t;
        new_t = tmp_t;

        let tmp_r = r - quotient * new_r;
        r = new_r;
        new_r = tmp_r;
    }

    if r > 1 {
        return None; 
    }

    if t < 0 {
        t += m;
    }
    Some(t)
}

fn encrypt_char(c: char, a: i32, b: i32) -> char {
    let m = 26;
    if c.is_ascii_alphabetic() {
        let x = (c.to_ascii_lowercase() as u8 - b'a') as i32;
        let y = (a * x + b).rem_euclid(m);
        (b'a' + y as u8) as char
    } else {
        c
    }
}

fn decrypt_char(c: char, a_inv: i32, b: i32) -> char {
    let m = 26;
    if c.is_ascii_alphabetic() {
        let y = (c.to_ascii_lowercase() as u8 - b'a') as i32;
        let x = a_inv * (y - b).rem_euclid(m);
        ((b'a' + x.rem_euclid(m) as u8) as char)
    } else {
        c
    }
}

pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if gcd(a, 26) != 1 {
        return Err(AffineCipherError::NotCoprime(a));
    }

    let mut result = String::new();
    let mut count = 0;

    for c in plaintext.chars() {
        if c.is_ascii_alphanumeric() {
            let enc = if c.is_ascii_alphabetic() {
                encrypt_char(c, a, b)
            } else {
                c 
            };
            result.push(enc);
            count += 1;

            if count % 5 == 0 {
                result.push(' ');
            }
        }
    }

    Ok(result.trim_end().to_string())
}

pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if gcd(a, 26) != 1 {
        return Err(AffineCipherError::NotCoprime(a));
    }

    let a_inv = mod_inv(a, 26).unwrap(); 
    let mut result = String::new();

    for c in ciphertext.chars() {
        if c.is_ascii_alphabetic() {
            let dec = decrypt_char(c, a_inv, b);
            result.push(dec);
        } else if c.is_ascii_digit() {
            result.push(c); 
        }

    }

    Ok(result)
}
