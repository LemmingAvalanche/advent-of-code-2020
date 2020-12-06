use std::fs;
mod day6;

fn main() {
    let input = fs::read_to_string(
        "<path to input>")
        .unwrap();
    println!("{}", day6::solve_part_1(&input));
}
