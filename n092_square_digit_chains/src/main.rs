use std::collections::HashMap;

type IntT = u32;

fn main() {
    const LIMIT: IntT = 10_000_000;
    let mut sum = 0;
    let mut cache = HashMap::new();
    for i in 1..LIMIT {
        let chain_end_for_i = chain_ends_in_39(i, &mut cache);
        // println!("{} {}", i, chain_end_for_i);
        if chain_end_for_i {
            sum += 1;
        }
    }
    println!("{} starting numbers arrived at 89", sum)
}

fn chain_ends_in_39(starting_number: IntT, cache: &mut HashMap<IntT, bool>) -> bool {
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
        assert!(!chain_ends_in_39(44, &mut HashMap::new()));
        assert!(chain_ends_in_39(85, &mut HashMap::new()));
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
