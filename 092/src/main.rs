use rayon::prelude::*;

type IntT = u32;

fn precompute_chain_end_results(limit: IntT) -> Vec<bool> {
    let mut results = vec![false; limit as usize + 1];

    for i in 1..=limit {
        let mut working_number = i;
        while working_number != 1 && working_number != 89 {
            working_number = square_sum(convert_int_to_vec(working_number));
        }
        results[i as usize] = working_number == 89;
    }

    results
}

fn factorial(n: IntT) -> IntT {
    (1..=n).product()
}

fn multinomial_coefficient(digits: &[IntT]) -> IntT {
    let mut counts = vec![0; 10];

    for &digit in digits {
        counts[digit as usize] += 1;
    }

    let factorials: Vec<IntT> = counts.into_iter().map(factorial).collect();
    factorial(7) / factorials.iter().product::<IntT>()
}

fn generate_combinations(n: usize, max_value: IntT, precomputed: &[bool]) -> Vec<IntT> {
    let mut result = Vec::new();

    fn helper(n: usize, max_value: IntT, start: IntT, depth: usize, current: &mut Vec<IntT>, precomputed: &[bool], result: &mut Vec<IntT>) {
        if depth == n {
            let sum = square_sum(current.to_owned());
            if precomputed[sum as usize] {
                let count = multinomial_coefficient(current);
                result.push(count);
            } else {
                result.push(0);
            }
        } else {
            for i in start..=max_value {
                current.push(i);
                helper(n, max_value, i, depth + 1, current, precomputed, result);
                current.pop();
            }
        }
    }

    helper(n, max_value, 0, 0, &mut Vec::with_capacity(n), precomputed, &mut result);
    result
}

fn main() {
    const PRECOMPUTED_LIMIT: IntT = 567; // sum(int(d)*int(d) for d in str(10_000_000-1)) = 567

    let precomputed = precompute_chain_end_results(PRECOMPUTED_LIMIT);

    let sum: IntT = generate_combinations(7, 9, &precomputed)
        .into_par_iter()
        .sum();

    println!("{} starting numbers arrived at 89", sum);
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
