use std::collections::HashSet;
use evalexpr::{eval, Value};
use itertools::Itertools;

fn main() {
    let operators = vec!['+', '-', '/', '*'];
    let digits = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9"];

    for digit_combination in digits.iter().combinations(4) {
        let mut targets = HashSet::new();
        // println!("{} {:?}", i, digit_combination);
        for digit_permutation in digit_combination.clone().into_iter().permutations(4) {
            // println!("{} {:?}: {:?}", i, digit_combination, digit_permutation);
            for operator_permutation in (0..operators.len() - 1).map(|_| operators.iter()).multi_cartesian_product() {
                let a: Vec<String> = digit_permutation.iter().map(|s| (*s).to_string()).collect::<Vec<String>>();
                let b: Vec<String> = operator_permutation.iter().map(|s| s.to_string()).collect::<Vec<String>>();
                let s = zip_string_vecs(&a, &b);
                // 0 "8/5/2*1"
                // x  0123456
                // 1 "(8/5)/2*1" 3) 0(
                // 2 "8/5/(2*1)" push) 4(
                // 3 "(8/5)/(2*1)" push) 4( 3) 0(
                // 4 "8/(5/2)*1" 5) 2(
                // 5 "(8/5/2)*1" 5) 0(
                // 6 "8/(5/2*1)" push) 2(
                // 7 "((8/5)/2)*1" 5) 3) 0( 0(
                // let mut s1 = s.clone();
                // s1.insert(3, ')');
                // s1.insert(0, '(');
                //
                // let mut s2 = s.clone();
                // s2.push(')');
                // s2.insert(4, '(');

                let mut s3 = s.clone();
                s3.push(')');
                s3.insert(4, '(');
                s3.insert(3, ')');
                s3.insert(0, '(');

                // let mut s4 = s.clone();
                // s4.insert(5, ')');
                // s4.insert(2, '(');
                //
                // let mut s5 = s.clone();
                // s5.insert(5, ')');
                // s5.insert(0, '(');
                //
                // let mut s6 = s.clone();
                // s6.push(')');
                // s6.insert(2, '(');

                let mut s7 = s.clone();
                s7.insert(5, ')');
                s7.insert(3, ')');
                s7.insert(0, '(');
                s7.insert(0, '(');

                let all_s = vec![s, s3, s7];

                for s in all_s.iter() {
                    // print!("{} {:?}: {:?}: {:?}", i, digit_combination, digit_permutation, s);
                    let ans = evaluate(s);
                    if let Some(ans) = ans {
                        // println!(" {}", ans);
                        if ans > 0 {
                            targets.insert(ans);
                        }
                    } else {
                        // println!();
                    }
                }
            }
        }
        let mut sorted_targets = Vec::from_iter(targets);
        sorted_targets.sort();
        let mut longest_continuous = 0;
        if sorted_targets[0] == 1 {
            for window in sorted_targets.windows(2) {
                if window[1] == window[0] + 1 {
                    longest_continuous += 1;
                } else {
                    break;
                }
            }
        }
        // dbg!(&sorted_targets);
        println!("{} {:?}", longest_continuous, &digit_combination);
    }
}


fn zip_string_vecs(a: &[String], b: &[String]) -> String {
    a.iter().cloned().interleave(b.iter().cloned()).collect()
}

fn evaluate(expr: &str) -> Option<i64> {
    let pos = expr.find(|c: char| c.is_ascii_digit()).unwrap();
    let mut expr = String::from(expr);
    expr.insert_str(pos+1, ".0");
    match eval(&expr) {
        Ok(d) => {
            match d {
                Value::String(_) => None,
                Value::Float(f) => {
                    if f == (f as u32) as f64 {
                        Some(f as i64)
                    } else {
                        None
                    }
                }
                Value::Int(d) => Some(d),
                Value::Boolean(_) => None,
                Value::Tuple(_) => None,
                Value::Empty => None,
            }
        }
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate() {
        assert_eq!(evaluate("1+1"), Some(2))
    }

    #[test]
    fn test_powerset() {
        let a = "1+2-3*4";
        let b = "() ";
        let c = format!("{}{}", a, b);
        let d = c.chars().powerset();
        for x in d {
            dbg!(&x);
        }
    }

    #[test]
    fn test_combinations() {
        let digits = vec!['1', '2', '3', '4', '5', '6', '7', '8', '9'];
        let mut s = 0;
        for combo in digits.iter().combinations(4) {
            for permutation in combo.iter().permutations(4) {
                println!("{} {:?} {:?}", s, &combo, &permutation);
                s += 1
            }
        }
    }

    #[test]
    fn test_combinations_with_replacement() {
        let operators = "+-*/";
        for combo in (0..operators.len() - 1).map(|_| operators.chars()).multi_cartesian_product() {
            println!("{:?}", combo);
        }
    }

    #[test]
    fn test_eval_float() {
        dbg!(evaluate("8+5+2+1"));
    }
}
