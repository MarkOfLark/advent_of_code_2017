use std::env;
use std::io::prelude::*;

fn get_puzzle_string() -> String {
    let mut puzzle_string = String::new();
    let args: Vec<_> = env::args().collect();
    if args.len() == 1 {
        panic!("Please provide the input or a file name");
    }
    else {
        if args.len() > 1 {
            // Try reading as a file. If it fails then assume argument is the input string as is.
            match std::fs::File::open(&args[1]) {
                Ok(mut f) => { f.read_to_string(&mut puzzle_string).expect("something went wrong reading the file"); }
                Err(_) => {
                    puzzle_string = args[1].clone();
                }
            };
        }
    }
    puzzle_string
}

pub fn advent_of_code_combined(pc : &Fn(&str) -> (String, String)) {
    let puzzle_string = get_puzzle_string();
    let (p1,p2) = pc(&puzzle_string);
    println!("Part 1: {}",p1);
    println!("Part 2: {}",p2);
}

pub fn advent_of_code(p1 : &Fn(&str) -> String, p2 : &Fn(&str) -> String) {
    let puzzle_string = get_puzzle_string();
    println!("Part 1: {}",p1(&puzzle_string));
    println!("Part 2: {}",p2(&puzzle_string));
}
