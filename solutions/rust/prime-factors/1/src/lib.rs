pub fn factors(n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut num = n;
    while num % 2 == 0 {
        factors.push(2);
        num /= 2;
    }
    let mut i = 3;
    while i * i <= num {
        while num % i == 0 {
            factors.push(i);
            num /= i;
        }
        i += 2;
    }
    if num > 2 {
        factors.push(num);
    }
    factors
}