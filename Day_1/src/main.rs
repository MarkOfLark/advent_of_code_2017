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
            -g                   'Use Gold star solution instead of Silver'
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

    match matches.occurrences_of("g") {
        1 => {
            println!("GOLD STAR SOLUTION: {}",0);
        },
        _ => {
            println!("SILVER STAR SOLUTION: {}",0);
        }
    }
}
