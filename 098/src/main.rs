/*
By replacing each of the letters in the word CARE with 1, 2, 9, and 6 respectively, we form a square number: 1296 = 362. What is remarkable is that, by using the same digital substitutions, the anagram, RACE, also forms a square number: 9216 = 962. We shall call CARE (and RACE) a square anagram word pair and specify further that leading zeroes are not permitted, neither may a different letter have the same digital value as another letter.

Using words.txt (right click and 'Save Link/Target As...'), a 16K text file containing nearly two-thousand common English words, find all the square anagram word pairs (a palindromic word is NOT considered to be an anagram of itself).

What is the largest square number formed by any member of such a pair?

NOTE: All anagrams formed must be contained in the given text file.
*/
use std::collections::{HashMap, HashSet};

fn read_words() -> Vec<String> {
    let words = include_str!("../p098_words.txt");
    words
        .split(',')
        .map(|w| w.trim_matches('"').to_string())
        .collect()
}

// create hashmap with key = length of word and value = vector of words of that length
fn create_word_map(words: Vec<String>) -> HashMap<usize, Vec<String>> {
    let mut word_map = HashMap::new();
    for word in words {
        let len = word.len();
        if len > 1 {
            word_map.entry(len).or_insert_with(Vec::new).push(word);
        }
    }
    word_map
}

fn create_length_map_of_squares(square_set: &HashSet<String>) -> HashMap<usize, HashSet<String>> {
    let mut length_map = HashMap::new();
    for square in square_set {
        let len = square.to_string().len();
        length_map
            .entry(len)
            .or_insert_with(HashSet::new)
            .insert(square.to_string());
    }
    length_map
}

fn create_set_of_squares(max_square: u64) -> HashSet<String> {
    let mut squares = HashSet::new();
    for i in 1..(max_square as f64).sqrt() as u64 {
        let square = i * i;
        squares.insert(square.to_string());
    }
    squares
}

fn is_anagram(word: &str, anagram: &str) -> bool {
    let mut word_chars = word.chars().collect::<Vec<char>>();
    let mut anagram_chars = anagram.chars().collect::<Vec<char>>();
    word_chars.sort();
    anagram_chars.sort();
    word_chars == anagram_chars
}

fn is_palindrome(word: &str) -> bool {
    let mut chars = word.chars().collect::<Vec<char>>();
    chars.reverse();
    let reversed = chars.iter().collect::<String>();
    word == reversed
}

fn create_map_of_anagrams(word_map: &HashMap<usize, Vec<String>>) -> HashMap<String, Vec<String>> {
    let mut anagram_map = HashMap::new();
    for words in word_map.values() {
        for word in words {
            // loop over all words of the same length
            for anagram in words {
                // loop over all words of the same length
                if word != anagram && is_anagram(word, anagram) && !is_palindrome(word) {
                    anagram_map
                        .entry(word.to_string())
                        .or_insert_with(Vec::new)
                        .push(anagram.to_string());
                }
            }
        }
    }
    anagram_map
}

fn is_digits_in_number_unique(digits: &str) -> bool {
    let mut digit_set = HashSet::new();
    for digit in digits.chars() {
        if digit_set.contains(&digit) {
            return false;
        }
        digit_set.insert(digit);
    }
    true
}

fn check(word: &str, anagrams: Vec<String>, squares: &HashSet<String>) -> Option<String> {
    let mut anagramic_squares = HashSet::new();
    for square in squares {
        // assign each letter in word to a digit
        let mut digit_map = HashMap::new();
        word.chars().zip(square.chars()).for_each(|(w, s)| {
            digit_map.insert(w, s);
        });

        for anagram in &anagrams {
            let mut anagram_digits = String::new();
            for c in anagram.chars() {
                anagram_digits.push(*digit_map.get(&c).unwrap());
            }

            if is_digits_in_number_unique(&anagram_digits) && squares.contains(&anagram_digits) {
                anagramic_squares.insert(anagram_digits);
            }
        }
    }

    // return the largest anagramic square number
    match anagramic_squares
        .iter()
        .map(|s| s.parse::<u64>().unwrap())
        .max()
    {
        None => {
            eprintln!("No anagramic squares found for {}", word);
            None
        }
        Some(d) => Some(d.to_string()),
    }
}

fn main() {
    let words = read_words();
    let start_time = std::time::Instant::now();
    let word_map = create_word_map(words);
    let anagram_map = create_map_of_anagrams(&word_map);
    dbg!(&anagram_map);
    let length_of_longest_word = anagram_map.keys().map(|w| w.len()).max().unwrap();
    dbg!(length_of_longest_word);
    let max_square = 10_u64.pow(length_of_longest_word as u32);
    let squares = create_set_of_squares(max_square);
    let square_length_map = create_length_map_of_squares(&squares);

    let mut largest_square = 0;
    let mut best_word = String::new();
    for (word, anagrams) in anagram_map {
        let length = word.len();
        let squares = square_length_map.get(&length).unwrap();
        let result = check(&word, anagrams, squares);
        if let Some(result) = result {
            println!("{}: {}", word, result);
            let square = result.parse::<u64>().unwrap();
            if square > largest_square {
                largest_square = square;
                best_word = word;
            }
        }
    }

    let elapsed = start_time.elapsed();
    println!("Elapsed: {}ms", elapsed.as_millis());
    println!(
        "The largest square number is {} for word {}",
        largest_square, best_word
    );
}
