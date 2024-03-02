use advent_of_code::problems;
use advent_of_code::utils::file_utils;
use std::process;

fn main() {
    let problem_data = file_utils::read_problem_file_contents(9).unwrap_or_else(|err| {
        println!("Problem loading data: {err}");
        process::exit(1);
    });
    problems::problem9::solve(&problem_data);
}
