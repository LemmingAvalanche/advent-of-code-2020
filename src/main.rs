use std::fs;
mod day5;

fn main() {
    let input = fs::read_to_string(
        "<put path to input file here>")
        .unwrap();
    let per = day5::input_generator(&input);
    println!("{}", day5::solve_part1(&per));
}
