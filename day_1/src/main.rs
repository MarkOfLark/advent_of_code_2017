extern crate aoch;

fn main() {
    aoch::advent_of_code(&part_one_solution, &part_two_solution);
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
