extern crate clap;

//use std::fs::File;
use std::io::prelude::*;

fn main() {
    let matches = clap::App::new("Advent of Code 2017 - Day 1")
        .version("1.0")
        .author("Mark E. McDermott")
        .about("Solution to Advent of Code 2017 - Day 1")
        .args_from_usage(
            "-f                  'Read input as file instead of as string'
            -x                   'Use part two solution'
            <INPUT>              'Sets the input file to use'")
        .get_matches();

    
    let mut captcha = String::new();
    match matches.occurrences_of("f") {
        0 => {
            captcha = matches.value_of("INPUT").unwrap().to_string();
        },
        _ => {
            let mut f = std::fs::File::open(matches.value_of("INPUT").unwrap()).expect("file not found");
            f.read_to_string(&mut captcha).expect("something went wrong reading the file");
        },
    };

    let total = match matches.occurrences_of("x") {
        1 => {
            part_two_solution(&captcha)
        },
        _ => {
            part_one_solution(&captcha)
        }
    };

    println!("{}",total);
}


fn part_one_solution(captcha: &str) -> u64 {
    let mut total : u64 = 0;
    let mut previous_char = captcha.chars().last().unwrap();
    for c in captcha.chars() {
        if c == previous_char {
            total = total + c.to_digit(10).unwrap() as u64;
        }
        previous_char = c;
    }
    total
}


fn part_two_solution(captcha: &str) -> u64 {
    let mut total : u64 = 0;

    let half_idx = captcha.chars().count()/2;
    let first_half = &captcha[0..half_idx];
    let second_half = &captcha[half_idx..captcha.chars().count()];

    for pair in first_half.chars().zip(second_half.chars()) {
        let (f, s) = pair;
        if f == s {
            total = total + 2*f.to_digit(10).unwrap() as u64;
        }
    }

    total
}
