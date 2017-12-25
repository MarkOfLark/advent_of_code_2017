extern crate aoch;


fn main() {
    aoch::advent_of_code(&part_one_solution, &part_two_solution);
}


fn get_jump_table(puzzle_input: &str) -> Vec<i32> {
    puzzle_input.split_whitespace()
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<i32>().unwrap())
                .collect()
}


fn part_one_solution(puzzle_input: &str) -> String {
    let mut jump_table = get_jump_table(puzzle_input);
    
    let mut counter = 0u32;
    let mut sp = 0i32;
    loop {
        let next_address = jump_table[sp as usize] + sp;
        
        jump_table[sp as usize] = jump_table[sp as usize] + 1;
        sp = next_address;

        counter += 1;

        if 0 > next_address || (jump_table.len() as i32)-1 < next_address {
            return counter.to_string();
        }
    }
}




fn part_two_solution(puzzle_input: &str) -> String {
    let mut jump_table = get_jump_table(puzzle_input);
    
    let mut counter = 0u32;
    let mut sp = 0i32;
    loop {
        let next_address = jump_table[sp as usize] + sp;
        
        if 3 <= jump_table[sp as usize] {
            jump_table[sp as usize] = jump_table[sp as usize] - 1;
        }
        else {
            jump_table[sp as usize] = jump_table[sp as usize] + 1;
        }
        sp = next_address;

        counter += 1;

        if 0 > next_address || (jump_table.len() as i32)-1 < next_address {
            return counter.to_string();
        }
    }
}
