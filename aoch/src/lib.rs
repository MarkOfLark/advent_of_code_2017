use std::env;
use std::io::prelude::*;

pub fn advent_of_code(p1 : &Fn(&str) -> u64, p2 : &Fn(&str) -> u64) {
    let args: Vec<_> = env::args().collect();
    if args.len() == 1 {
        println!("Please provide the input or a file name. Optionally provide 1 or 2 to just run one part of the puzzle.");
    }
    else {
        let mut puzzle_string = String::new();
        if args.len() > 1 {
            // Try reading as a file. If it fails then assume argument is the input string as is.
            match std::fs::File::open(&args[1]) {
                Ok(mut f) => { f.read_to_string(&mut puzzle_string).expect("something went wrong reading the file"); }
                Err(_) => {
                    puzzle_string = args[1].clone();
                }
            };
        }

        if args.len() > 2 {
            match args[2].parse::<i32>().unwrap() {
                1 => println!("{}", p1(&puzzle_string)),
                2 => println!("{}", p2(&puzzle_string)),
                _ => (),
            };
        }
        else {
            println!("Part 1: {}",p1(&puzzle_string));
            println!("Part 2: {}",p2(&puzzle_string));
        }
    }
}
