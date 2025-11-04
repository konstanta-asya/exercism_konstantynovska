#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list.len() == second_list.len() {
        for i in 0..first_list.len() {
            if first_list[i] != second_list[i] {
                return Comparison::Unequal;
            }
        }

        Comparison::Equal
    } else {
        let bigger = first_list.len() > second_list.len();
        let big_arr = if bigger { first_list } else { second_list };
        let small_arr = if bigger { second_list } else { first_list };

        for i in 0..=big_arr.len() - small_arr.len() {
            let mut equal = true;
            for j in 0..small_arr.len() {
                if big_arr[i + j] != small_arr[j] {
                    equal = false;
                    break;
                }
            }

            if equal {
                if bigger {
                    return Comparison::Superlist;
                } else {
                    return Comparison::Sublist;
                }
            }
        }

        Comparison::Unequal
    }
}
