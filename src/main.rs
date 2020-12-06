use std::fs;
mod day6;

fn main() {
    // I just update this function for each problem that I bother to solve,
    // setting the input file path and the function call.
    let input = fs::read_to_string(
        "<path to input>")
        .unwrap();
    println!("{}", day6::solve_part_2(&input));
}
