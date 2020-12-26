use std::fs;
mod day18;

fn main() {
    // I just update this function for each problem that I bother to solve,
    // setting the input file path and the function call.
    let input = fs::read_to_string(
        "<path to input>")
        .unwrap();
    println!("{}", day18::sum_lines(&input));
}
