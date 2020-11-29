use std::fs;
use std::collections::HashSet;

fn main() {
    //let limit = 50;
    let limit = 50_000_000;

    // large_primes.txt contains all primes below 100 million
    let filename = "../additionals/large_primes.txt";
    let prime_file_content = fs::read_to_string(filename).expect(&*format!("Unable to read file {}", filename));

    let primes: Vec<u128> = prime_file_content
        .lines()
        .map(|x: &str| x.parse().unwrap())
        .filter(|x: &u128| x < &limit)
        .collect();

    let primes_squared: Vec<u128> = primes.iter()
        .map(|x| x.pow(2))
        .filter(|x| x < &limit)
        .collect();


    let primes_cubed: Vec<u128> = primes.iter()
        .map(|x| x.pow(3))
        .filter(|x| x < &limit)
        .collect();


    let primes_fourth: Vec<u128> = primes.iter()
        .map(|x| x.pow(4))
        .filter(|x| x < &limit)
        .collect();

    println!("Number of primes squared: {}", primes_squared.len());
    println!("Number of primes cubed: {}", primes_cubed.len());
    println!("Number of primes fourth: {}", primes_fourth.len());

    let mut triples_below_limit = HashSet::new();

    for a_squared in primes_squared.iter()  {
        for b_squared in primes_cubed.iter() {
            for c_squared in primes_fourth.iter() {
                let candidate: u128 = a_squared + b_squared + c_squared;
                if candidate < limit {
                    triples_below_limit.insert(candidate);
                }
            }
        }
    }

    println!("{}", triples_below_limit.len());
}
