extern crate aoch;
use std::collections::HashMap;


fn main() {
    aoch::advent_of_code_combined(&common_solution);
}

fn common_solution(puzzle_input: &str) -> (String,String) {
    let mut register_table = HashMap::new();
    let mut max_register_ever = 0;

    for line in puzzle_input.lines() {
        let token : Vec<&str> = line.split_whitespace()
                                .map(|s| s.trim())
                                .filter(|s| !s.is_empty())
                                .collect();
        println!("{:?}",token);

        let operand = token[0];
        let operation = token[1];
        let operation_value = token[2].parse::<i32>().expect("operation value invalid");
        let conditioned = token[4];
        let condition = token[5];
        let condition_value = token[6].parse::<i32>().expect("condition value invalid");

        let source_register = *register_table.entry(conditioned).or_insert(0);
        let should_operate = match condition {
            ">"  => (source_register > condition_value),
            "<"  => (source_register < condition_value),
            "<=" => (source_register <= condition_value),
            ">=" => (source_register >= condition_value),
            "!=" => (source_register != condition_value),
            "==" => (source_register == condition_value),
            _    => false,
        };
        if should_operate {
            let mut target_register = register_table.entry(operand).or_insert(0);
            match operation {
                "inc" => *target_register += operation_value,
                "dec" => *target_register -= operation_value,
                _     => (),
            }
        }

        let max_register_now = *register_table.values().max().expect("No max value");
        if max_register_now > max_register_ever {
            max_register_ever = max_register_now;
        }
    }

    let max_register = *register_table.values().max().expect("No max value");

    (max_register.to_string(),max_register_ever.to_string())
}
