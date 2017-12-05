extern crate aoch;

fn main() {
    aoch::advent_of_code(&part_one_solution, &part_two_solution);
}


fn parse_spreadsheet_line(line: &str) -> Vec<u64> {
    line.split_whitespace()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u64>().unwrap())
        .collect()
}


fn part_one_solution(spreadsheet: &str) -> u64 {
    let mut total = 0u64;
    for line in spreadsheet.lines() {
        let cell_values : Vec<u64> = parse_spreadsheet_line(&line);
        total += cell_values.iter().max().expect("no max") - cell_values.iter().min().expect("no min");
    }
    total
}


fn part_two_solution(spreadsheet: &str) -> u64 {
    let mut total = 0u64;
    for line in spreadsheet.lines() {
        let mut cell_values : Vec<u64> = parse_spreadsheet_line(&line);
        cell_values.sort_unstable_by(|a,b| b.cmp(a));

        let find_divisible = | | {
            for num_idx in 0..cell_values.len() {
                for den_idx in num_idx+1..cell_values.len() {
                    if 0 == cell_values[num_idx] % cell_values[den_idx] {
                        return cell_values[num_idx]/cell_values[den_idx];
                    };
                }
            }
            0
        };

        total += find_divisible();
    }
    total
}