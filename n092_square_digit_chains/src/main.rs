use std::cell::RefCell;
use std::collections::HashMap;
use rayon::prelude::*;

type IntT = u32;

fn main() {
    const LIMIT: IntT = 10_000_000;
    let sum = (1..LIMIT).into_par_iter()
        .map(chain_ends_in_89)
        .filter(|&ends_in_89| ends_in_89)
        .count();
    println!("{} starting numbers arrived at 89", sum);
}

fn chain_ends_in_89(starting_number: IntT) -> bool {
    thread_local! {
        static CACHE: RefCell<HashMap<IntT, bool>> = RefCell::new(HashMap::new());
    }

    fn chain_ends_in_39_inner(starting_number: IntT, cache: &mut HashMap<IntT, bool>) -> bool {
        let mut working_number = starting_number;
        loop {
            if working_number == 1 {
                cache.insert(starting_number, false);
                return false;
            } else if working_number == 89 {
                cache.insert(starting_number, true);
                return true;
            } else {
                if let Some(&cached_result) = cache.get(&working_number) {
                    cache.insert(starting_number, cached_result);
                    return cached_result;
                }
                working_number = square_sum(convert_int_to_vec(working_number));
            }
        }
    }

    CACHE.with(|cache| chain_ends_in_39_inner(starting_number, &mut cache.borrow_mut()))
}

fn square_sum(n: Vec<IntT>) -> IntT {
    n.into_iter().fold(0, |acc, d| acc + d*d)
}

fn convert_int_to_vec(n: IntT) -> Vec<IntT> {
    let n_str = n.to_string();
    n_str.chars().map(|c| {
        c.to_digit(10).unwrap()
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chain() {
        assert!(!chain_ends_in_89(44));
        assert!(chain_ends_in_89(85));
    }

    #[test]
    fn test_convert_int_to_vec() {
        assert_eq!(convert_int_to_vec(123), vec![1, 2, 3]);
        assert_eq!(convert_int_to_vec(3), vec![3]);
    }

    #[test]
    fn test_square_sum() {
        assert_eq!(square_sum(vec![3, 2]), 13);
        assert_eq!(square_sum(vec![1]), 1);
    }
}
