extern crate aoch;

use std::collections::BTreeSet;
use std::iter::FromIterator;

fn main() {
    aoch::advent_of_code(&part_one_solution, &part_two_solution);
}


fn part_one_solution(passphrase_list: &str) -> String {
    let mut total = 0;
    for word_list in passphrase_list.lines() {
        let mut word_validator = BTreeSet::new();
        let word_list : Vec<&str> = word_list.split_whitespace().collect();

        let mut valid = true;
        for word in word_list.iter() {
            if !word_validator.insert(*word) {
                valid = false;
            }
        }

        if valid {
            total += 1;
        }
    }
    total.to_string()
}




fn part_two_solution(passphrase_list: &str) -> String {
    let mut total = 0;
    for word_list in passphrase_list.lines() {
        let mut word_validator = BTreeSet::new();
        let word_list : Vec<&str> = word_list.split_whitespace().collect();

        let mut valid = true;
        for word in word_list.iter() {
            let mut chars: Vec<char> = word.chars().collect();
            chars.sort_by(|a,b| b.cmp(a));

            if !word_validator.insert(String::from_iter(chars)) {
                valid = false;
            }
        }

        if valid {
            total += 1;
        }
    }
    total.to_string()
}
