/*
The proper divisors of a number are all the divisors excluding the number itself. For example, the proper divisors of 28 are 1, 2, 4, 7, and 14. As the sum of these divisors is equal to 28, we call it a perfect number.

Interestingly the sum of the proper divisors of 220 is 284 and the sum of the proper divisors of 284 is 220, forming a chain of two numbers. For this reason, 220 and 284 are called an amicable pair.

Perhaps less well known are longer chains. For example, starting with 12496, we form a chain of five numbers:

12496 → 14288 → 15472 → 14536 → 14264 (→ 12496 → ...)

Since this chain returns to its starting point, it is called an amicable chain.

Find the smallest member of the longest amicable chain with no element exceeding one million.

Answer can be found on wikipedia: https://en.wikipedia.org/wiki/Sociable_number
*/
use rayon::prelude::*;

fn main() {
    let chain = longest_amicable_chain();
    println!("The longest amicable chain with no element exceeding one million is {:?}", chain);
    println!("The smallest member of the longest amicable chain with no element exceeding one million is {}", smallest_element_in_chain(chain));
}

fn smallest_element_in_chain(chain: Vec<u64>) -> u64 {
    let mut smallest = chain[0];
    for i in chain {
        if i < smallest {
            smallest = i;
        }
    }
    smallest
}


fn longest_amicable_chain() -> Vec<u64> {
    (1..1_000_000)
        .into_par_iter()
        .map(amicable_chain)
        .reduce_with(|a, b| if a.len() > b.len() { a } else { b })
        .unwrap_or_default()
}

fn amicable_chain(n: u64) -> Vec<u64> {
    // println!("Calculating chain for {}", n);
    let mut chain = Vec::new();
    chain.push(n);

    let mut next = sum_of_proper_divisors(n);
    while next != n && next <= 1_000_000 {
        chain.push(next);
        next = sum_of_proper_divisors(next);
        if chain.len() > 28 {
            return Vec::new();
        }
    }
    if next >= 1_000_000 {
        return Vec::new();
    }
    if *chain.last().unwrap() == 0 {
        return Vec::new();
    }
    chain
}

fn sum_of_proper_divisors(n: u64) -> u64 {
    proper_divisors(n).iter().sum()
}

fn proper_divisors(n: u64) -> Vec<u64> {
    let mut divisors = Vec::new();

    for i in 1..n {
        if n % i == 0 {
            divisors.push(i);
        }
    }
    divisors
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test() {
        dbg!(amicable_chain(633420));
    }

    #[test]
    fn test_amicable_chain() {
        assert_eq!(amicable_chain(220), vec![220, 284]);
        assert_eq!(amicable_chain(284), vec![284, 220]);
        assert_eq!(amicable_chain(12496), vec![12496, 14288, 15472, 14536, 14264]);
    }

    #[test]
    fn test_sum_of_proper_divisors() {
        assert_eq!(sum_of_proper_divisors(1), 1-1);
        assert_eq!(sum_of_proper_divisors(2), 3-2);
        assert_eq!(sum_of_proper_divisors(3), 4-3);
        assert_eq!(sum_of_proper_divisors(4), 7-4);
        assert_eq!(sum_of_proper_divisors(5), 6-5);
        assert_eq!(sum_of_proper_divisors(6), 12-6);
        assert_eq!(sum_of_proper_divisors(7), 8-7);
        assert_eq!(sum_of_proper_divisors(8), 15-8);
        assert_eq!(sum_of_proper_divisors(9), 13-9);
        assert_eq!(sum_of_proper_divisors(10), 18-10);
        assert_eq!(sum_of_proper_divisors(11), 12-11);
        assert_eq!(sum_of_proper_divisors(12), 28-12);
        assert_eq!(sum_of_proper_divisors(13), 14-13);
        assert_eq!(sum_of_proper_divisors(14), 24-14);
        assert_eq!(sum_of_proper_divisors(15), 24-15);
        assert_eq!(sum_of_proper_divisors(16), 31-16);
        assert_eq!(sum_of_proper_divisors(17), 18-17);
        assert_eq!(sum_of_proper_divisors(18), 39-18);
        assert_eq!(sum_of_proper_divisors(19), 20-19);
        assert_eq!(sum_of_proper_divisors(20), 42-20);
        assert_eq!(sum_of_proper_divisors(21), 32-21);
        assert_eq!(sum_of_proper_divisors(22), 36-22);
        assert_eq!(sum_of_proper_divisors(23), 24-23);
        assert_eq!(sum_of_proper_divisors(24), 60-24);
        assert_eq!(sum_of_proper_divisors(25), 31-25);
        assert_eq!(sum_of_proper_divisors(26), 42-26);
        assert_eq!(sum_of_proper_divisors(27), 40-27);
        assert_eq!(sum_of_proper_divisors(28), 56-28);
        assert_eq!(sum_of_proper_divisors(29), 30-29);
        assert_eq!(sum_of_proper_divisors(30), 72-30);
        assert_eq!(sum_of_proper_divisors(31), 32-31);
        assert_eq!(sum_of_proper_divisors(32), 63-32);
        assert_eq!(sum_of_proper_divisors(33), 48-33);
        assert_eq!(sum_of_proper_divisors(34), 54-34);
        assert_eq!(sum_of_proper_divisors(35), 48-35);
        assert_eq!(sum_of_proper_divisors(36), 91-36);
        assert_eq!(sum_of_proper_divisors(37), 38-37);
        assert_eq!(sum_of_proper_divisors(38), 60-38);
        assert_eq!(sum_of_proper_divisors(39), 56-39);
        assert_eq!(sum_of_proper_divisors(40), 90-40);
        assert_eq!(sum_of_proper_divisors(41), 42-41);
        assert_eq!(sum_of_proper_divisors(42), 96-42);
        assert_eq!(sum_of_proper_divisors(43), 44-43);
        assert_eq!(sum_of_proper_divisors(44), 84-44);
        assert_eq!(sum_of_proper_divisors(45), 78-45);
        assert_eq!(sum_of_proper_divisors(46), 72-46);
        assert_eq!(sum_of_proper_divisors(47), 48-47);
        assert_eq!(sum_of_proper_divisors(48), 124-48);
        assert_eq!(sum_of_proper_divisors(49), 57-49);
        assert_eq!(sum_of_proper_divisors(50), 93-50);
        assert_eq!(sum_of_proper_divisors(51), 72-51);
        assert_eq!(sum_of_proper_divisors(52), 98-52);
        assert_eq!(sum_of_proper_divisors(53), 54-53);
        assert_eq!(sum_of_proper_divisors(54), 120-54);
        assert_eq!(sum_of_proper_divisors(55), 72-55);
        assert_eq!(sum_of_proper_divisors(56), 120-56);
        assert_eq!(sum_of_proper_divisors(57), 80-57);
        assert_eq!(sum_of_proper_divisors(58), 90-58);
        assert_eq!(sum_of_proper_divisors(59), 60-59);
        assert_eq!(sum_of_proper_divisors(60), 168-60);
    }

    #[rstest(
    input, expected,
    case(1,	vec![]),
    case(2,	vec![1]),
    case(3,	vec![1]),
    case(4,	vec![1, 2]),
    case(5,	vec![1]),
    case(6,	vec![1, 2, 3]),
    case(7,	vec![1]),
    case(8,	vec![1, 2, 4]),
    case(9,	vec![1, 3]),
    case(10, vec![1, 2, 5]),
    case(11, vec![1]),
    case(12, vec![1, 2, 3, 4, 6]),
    case(13, vec![1]),
    case(14, vec![1, 2, 7]),
    case(15, vec![1, 3, 5]),
    case(16, vec![1, 2, 4, 8]),
    case(17, vec![1]),
    case(18, vec![1, 2, 3, 6, 9]),
    case(19, vec![1]),
    case(20, vec![1, 2, 4, 5, 10]),
    case(21, vec![1, 3, 7]),
    case(22, vec![1, 2, 11]),
    case(23, vec![1]),
    case(24, vec![1, 2, 3, 4, 6, 8, 12]),
    case(25, vec![1, 5]),
    case(26, vec![1, 2, 13]),
    case(27, vec![1, 3, 9]),
    case(28, vec![1, 2, 4, 7, 14]),
    case(29, vec![1]),
    case(30, vec![1, 2, 3, 5, 6, 10, 15]),
    case(31, vec![1]),
    case(32, vec![1, 2, 4, 8, 16]),
    case(33, vec![1, 3, 11]),
    case(34, vec![1, 2, 17]),
    case(35, vec![1, 5, 7]),
    case(36, vec![1, 2, 3, 4, 6, 9, 12, 18]),
    case(37, vec![1]),
    case(38, vec![1, 2, 19]),
    case(39, vec![1, 3, 13]),
    case(40, vec![1, 2, 4, 5, 8, 10, 20]),
    case(41, vec![1]),
    case(42, vec![1, 2, 3, 6, 7, 14, 21]),
    case(43, vec![1]),
    case(44, vec![1, 2, 4, 11, 22]),
    case(45, vec![1, 3, 5, 9, 15]),
    case(46, vec![1, 2, 23]),
    case(47, vec![1]),
    case(48, vec![1, 2, 3, 4, 6, 8, 12, 16, 24]),
    case(49, vec![1, 7]),
    case(50, vec![1, 2, 5, 10, 25]),
    case(51, vec![1, 3, 17]),
    case(52, vec![1, 2, 4, 13, 26]),
    case(53, vec![1]),
    case(54, vec![1, 2, 3, 6, 9, 18, 27]),
    case(55, vec![1, 5, 11]),
    case(56, vec![1, 2, 4, 7, 8, 14, 28]),
    case(57, vec![1, 3, 19]),
    case(58, vec![1, 2, 29]),
    case(59, vec![1]),
    case(60, vec![1, 2, 3, 4, 5, 6, 10, 12, 15, 20, 30]),
    case(61, vec![1]),
    case(62, vec![1, 2, 31]),
    case(63, vec![1, 3, 7, 9, 21]),
    case(64, vec![1, 2, 4, 8, 16, 32]),
    case(65, vec![1, 5, 13]),
    case(66, vec![1, 2, 3, 6, 11, 22, 33]),
    case(67, vec![1]),
    case(68, vec![1, 2, 4, 17, 34]),
    case(69, vec![1, 3, 23]),
    case(70, vec![1, 2, 5, 7, 10, 14, 35]),
    case(71, vec![1]),
    case(72, vec![1, 2, 3, 4, 6, 8, 9, 12, 18, 24, 36]),
    case(73, vec![1]),
    case(74, vec![1, 2, 37]),
    case(75, vec![1, 3, 5, 15, 25]),
    case(76, vec![1, 2, 4, 19, 38]),
    case(77, vec![1, 7, 11]),
    case(78, vec![1, 2, 3, 6, 13, 26, 39]),
    case(79, vec![1]),
    case(80, vec![1, 2, 4, 5, 8, 10, 16, 20, 40]),
    case(81, vec![1, 3, 9, 27]),
    case(82, vec![1, 2, 41]),
    case(83, vec![1]),
    case(84, vec![1, 2, 3, 4, 6, 7, 12, 14, 21, 28, 42]),
    case(85, vec![1, 5, 17]),
    case(86, vec![1, 2, 43]),
    case(87, vec![1, 3, 29]),
    case(88, vec![1, 2, 4, 8, 11, 22, 44]),
    case(89, vec![1]),
    case(90, vec![1, 2, 3, 5, 6, 9, 10, 15, 18, 30, 45]),
    case(91, vec![1, 7, 13]),
    case(92, vec![1, 2, 4, 23, 46]),
    case(93, vec![1, 3, 31]),
    case(94, vec![1, 2, 47]),
    case(95, vec![1, 5, 19]),
    case(96, vec![1, 2, 3, 4, 6, 8, 12, 16, 24, 32, 48]),
    case(97, vec![1]),
    case(98, vec![1, 2, 7, 14, 49]),
    case(99, vec![1, 3, 9, 11, 33]),
    case(100, vec![1, 2, 4, 5, 10, 20, 25, 50]),
    case(101, vec![1]),
    case(102, vec![1, 2, 3, 6, 17, 34, 51]),
    case(103, vec![1]),
    case(104, vec![1, 2, 4, 8, 13, 26, 52]),
    case(105, vec![1, 3, 5, 7, 15, 21, 35]),
    case(106, vec![1, 2, 53]),
    case(107, vec![1]),
    case(108, vec![1, 2, 3, 4, 6, 9, 12, 18, 27, 36, 54]),
    case(109, vec![1]),
    case(110, vec![1, 2, 5, 10, 11, 22, 55]),
    case(111, vec![1, 3, 37]),
    case(112, vec![1, 2, 4, 7, 8, 14, 16, 28, 56]),
    case(113, vec![1]),
    case(114, vec![1, 2, 3, 6, 19, 38, 57]),
    case(115, vec![1, 5, 23]),
    case(116, vec![1, 2, 4, 29, 58]),
    case(117, vec![1, 3, 9, 13, 39]),
    case(118, vec![1, 2, 59]),
    case(119, vec![1, 7, 17]),
    case(120, vec![1, 2, 3, 4, 5, 6, 8, 10, 12, 15, 20, 24, 30, 40, 60]),
    case(220, vec![1, 2, 4, 5, 10, 11, 20, 22, 44, 55, 110]),
    case(284, vec![1, 2, 4, 71, 142]),
    )]
    fn test_divisors(input: u64, expected: Vec<u64>) {
        let mut divisors = proper_divisors(input);
        divisors.sort();
        assert_eq!(divisors, expected);
    }
}
