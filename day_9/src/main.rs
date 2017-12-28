extern crate aoch;
use std::collections::HashMap;

fn main() {
    aoch::advent_of_code_combined(&common_solution);
}

enum State {
    None,
    Group,
    Garbage,
    Skip,
}

fn common_solution(puzzle_input: &str) -> (String,String) {
    let mut score = 0u32;
    let mut depth = 0;
    let mut garbage_count = 0u32;

    let mut state = Vec::new();
    state.push(State::None);

    for c in puzzle_input.chars() {
        match *state.last().expect("Empty Stack") {
            State::None => {
                match c {
                    '{' => {
                        depth += 1;
                        state.push(State::Group);
                    },
                    '<' => {
                        state.push(State::Garbage);
                    },
                    '!' => {
                        state.push(State::Skip);
                    },
                    _ => {},
                };
            },
            State::Group => {
                match c {
                    '{' => {
                        depth += 1;
                        state.push(State::Group);
                    },
                    '<' => {
                        state.push(State::Garbage);
                    },
                    '!' => {
                        state.push(State::Skip);
                    },
                    '}' => {
                        state.pop().expect("Failed to close group");
                        score += depth;
                        depth -= 1;
                    }
                    _ => {},
                };
            },
            State::Garbage => {                
                match c {
                    '!' => {
                        state.push(State::Skip);
                    },
                    '>' => {
                        state.pop().expect("Failed to close garbage");
                    }
                    _ => {
                        garbage_count += 1;
                    },
                };
            },
            State::Skip => {
                state.pop();
            },
        };
    }

    (score.to_string(),garbage_count.to_string())
}
