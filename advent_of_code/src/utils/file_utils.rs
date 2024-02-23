use std::{error::Error, fs};

pub fn read_problem_file_contents(problem_number: i8) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(format!("resources/day{problem_number}.txt"))?;
    Ok(contents)
}
