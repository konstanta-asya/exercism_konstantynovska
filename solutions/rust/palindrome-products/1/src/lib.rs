use std::collections::HashSet;
use std::cmp::{min, max};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palindrome {
    value: u64,
    factors: HashSet<(u64, u64)>,
}

impl Palindrome {
    pub fn new(value: u64) -> Self {
        Palindrome {
            value,
            factors: HashSet::new(),
        }
    }

    pub fn add_factor(&mut self, factor_a: u64, factor_b: u64) {
        let factor_pair = (min(factor_a, factor_b), max(factor_a, factor_b));
        self.factors.insert(factor_pair);
    }

    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn into_factors(self) -> HashSet<(u64, u64)> {
        self.factors
    }
}

fn is_palindrome(n: u64) -> bool {
    let s = n.to_string();
    s == s.chars().rev().collect::<String>()
}

pub fn palindrome_products(min_factor: u64, max_factor: u64) -> Option<(Palindrome, Palindrome)> {
    if min_factor > max_factor {
        return None;
    }

    let mut smallest_palindrome: Option<Palindrome> = None;
    let mut largest_palindrome: Option<Palindrome> = None;

    for a in min_factor..=max_factor {
        for b in a..=max_factor {
            let product = a * b;

            if is_palindrome(product) {
                if smallest_palindrome.is_none() {
                    let mut palindrome = Palindrome::new(product);
                    palindrome.add_factor(a, b);
                    smallest_palindrome = Some(palindrome.clone());
                    largest_palindrome = Some(palindrome);
                    continue;
                }

                let current_smallest = smallest_palindrome.as_mut().unwrap();
                let current_largest = largest_palindrome.as_mut().unwrap();

                match product.cmp(&current_smallest.value) {
                    std::cmp::Ordering::Less => {
                        let mut new_smallest = Palindrome::new(product);
                        new_smallest.add_factor(a, b);
                        smallest_palindrome = Some(new_smallest);
                    }
                    std::cmp::Ordering::Equal => {
                        current_smallest.add_factor(a, b);
                    }
                    std::cmp::Ordering::Greater => {}
                }

                match product.cmp(&current_largest.value) {
                    std::cmp::Ordering::Greater => {
                        let mut new_largest = Palindrome::new(product);
                        new_largest.add_factor(a, b);
                        largest_palindrome = Some(new_largest);
                    }
                    std::cmp::Ordering::Equal => {
                        current_largest.add_factor(a, b);
                    }
                    std::cmp::Ordering::Less => {}
                }
            }
        }
    }

    match (smallest_palindrome, largest_palindrome) {
        (Some(small), Some(large)) => Some((small, large)),
        _ => None,
    }
}
