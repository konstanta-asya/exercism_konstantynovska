#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }

    let mut sum = 1;
    let sqrt = (num as f64).sqrt() as u64;

    for i in 2..=sqrt {
        if num % i == 0 {
            sum += i;
            let pair = num / i;
            if pair != i {
                sum += pair;
            }
        }
    }

    if num == 1 {
        sum = 0;
    }

    Some(match sum.cmp(&num) {
        std::cmp::Ordering::Equal => Classification::Perfect,
        std::cmp::Ordering::Greater => Classification::Abundant,
        std::cmp::Ordering::Less => Classification::Deficient,
    })
}
