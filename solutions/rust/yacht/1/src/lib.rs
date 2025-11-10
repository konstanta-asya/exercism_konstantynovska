use std::collections::HashMap;

#[derive(Debug)]
pub enum Category {
    Ones,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    FullHouse,
    FourOfAKind,
    LittleStraight,
    BigStraight,
    Choice,
    Yacht,
}

type Dice = [u8; 5];

pub fn score(dice: Dice, category: Category) -> u8 {
    let mut counts = HashMap::new();
    for &d in &dice {
        *counts.entry(d).or_insert(0) += 1;
    }

    match category {
        Category::Ones => dice.iter().filter(|&&d| d == 1).sum(),
        Category::Twos => dice.iter().filter(|&&d| d == 2).map(|&d| d).sum(),
        Category::Threes => dice.iter().filter(|&&d| d == 3).map(|&d| d).sum(),
        Category::Fours => dice.iter().filter(|&&d| d == 4).map(|&d| d).sum(),
        Category::Fives => dice.iter().filter(|&&d| d == 5).map(|&d| d).sum(),
        Category::Sixes => dice.iter().filter(|&&d| d == 6).map(|&d| d).sum(),

        Category::FullHouse => {
            let values: Vec<_> = counts.values().cloned().collect();
            if values.contains(&3) && values.contains(&2) {
                dice.iter().sum()
            } else {
                0
            }
        }

        Category::FourOfAKind => {
            for (&die, &count) in &counts {
                if count >= 4 {
                    return die * 4;
                }
            }
            0
        }

        Category::LittleStraight => {
            let mut sorted = dice.clone();
            sorted.sort();
            if sorted == [1, 2, 3, 4, 5] {
                30
            } else {
                0
            }
        }

        Category::BigStraight => {
            let mut sorted = dice.clone();
            sorted.sort();
            if sorted == [2, 3, 4, 5, 6] {
                30
            } else {
                0
            }
        }

        Category::Choice => dice.iter().sum(),

        Category::Yacht => {
            if counts.values().any(|&v| v == 5) {
                50
            } else {
                0
            }
        }
    }
}
