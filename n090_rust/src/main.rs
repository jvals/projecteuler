use std::sync::{Arc, Mutex};
use std::thread;

fn dice_assignments() -> Vec<Vec<u8>> {
    let numbers = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let dice_assignments = Arc::new(Mutex::new(Vec::new()));
    let mut threads = Vec::new();

    for i in 0..5 {
        let assignments = dice_assignments.clone();
        let t = thread::spawn(move || {
            let mut thread_assignments = Vec::new();
            for j in (i + 1)..6 {
                for k in (j + 1)..7 {
                    for l in (k + 1)..8 {
                        for m in (l + 1)..9 {
                            for n in (m + 1)..10 {
                                let assignment = vec![
                                    numbers[i],
                                    numbers[j],
                                    numbers[k],
                                    numbers[l],
                                    numbers[m],
                                    numbers[n],
                                ];
                                thread_assignments.push(assignment);
                            }
                        }
                    }
                }
            }
            assignments.lock().unwrap().extend(thread_assignments);
        });
        threads.push(t);
    }

    for t in threads {
        t.join().unwrap();
    }

    Arc::try_unwrap(dice_assignments).unwrap().into_inner().unwrap()
}

fn can_form_squares(assignment1: &[u8], assignment2: &[u8]) -> bool {
    let squares: Vec<u8> = vec![1, 4, 9, 16, 25, 36, 49, 64, 81];
    let mut seen_squares = vec![false; squares.len()];

    for num1 in assignment1.iter() {
        for num2 in assignment2.iter() {
            if *num1 != *num2 {
                check(&squares, &mut seen_squares, num1, num2);
                check(&squares, &mut seen_squares, num2, num1);
            }
        }
    }

    if seen_squares.iter().any(|&x| !x) {
        return false;
    }
    true
}

fn check(squares: &[u8], seen_squares: &mut [bool], num1: &u8, num2: &u8) {
    let combined_num = format!("{}{}", num1, num2);
    let combined_num = combined_num.parse::<u8>().unwrap();
    if squares.contains(&combined_num) {
        seen_squares[squares.iter().position(|&x| x == combined_num).unwrap()] = true;
    }
}

fn extend_assignments(assignments: &mut Vec<Vec<u8>>) {
    for assignment in assignments {
        if assignment.contains(&6) && !assignment.contains(&9) {
            assignment.push(9);
        } else if assignment.contains(&9) && !assignment.contains(&6) {
            assignment.push(6);
        }
    }
}

fn main() {
    let mut assignments = dice_assignments();
    let num_assignments = assignments.len();
    extend_assignments(&mut assignments);
    println!("The number of assignments: {}", num_assignments);
    let mut sum = 0;

    for i in 0..num_assignments {
        for j in (i + 1)..num_assignments {
            if can_form_squares(&assignments[i], &assignments[j]) {
                println!("Found two assignments that can form all the square numbers below 100:");
                println!("{:?}", assignments[i]);
                println!("{:?}", assignments[j]);
                sum += 1;
            }
        }
    }
    println!("{}", sum)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_form_squares_true() {
        let assignment1 = vec![0, 5, 6, 7, 8, 9];
        let assignment2 = vec![1, 2, 3, 4, 8, 9];
        assert!(can_form_squares(&assignment1, &assignment2));
    }

    #[test]
    fn test_can_form_squares_false() {
        let assignment1 = vec![0, 1, 2, 3, 4, 5];
        let assignment2 = vec![5, 4, 3, 2, 1, 0];
        assert!(!can_form_squares(&assignment1, &assignment2));
    }

    #[test]
    fn test_can_form_squares_with_flips() {
        let assignment1 = vec![0, 5, 6, 7, 8, 9];
        let assignment2 = vec![1, 2, 3, 4, 6, 7];
        assert!(can_form_squares(&assignment1, &assignment2));
    }
}

