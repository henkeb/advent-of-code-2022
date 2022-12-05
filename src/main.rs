use exitfailure::ExitFailure;
use std::env;
use std::fs;

fn main() -> Result<(), ExitFailure> {
    let args: Vec<String> = env::args().collect();
    if let Ok(input) = fs::read_to_string(&args[1]) {
        println!("{:?}", advent_of_code_2022::day5::calculate(&input, true))
    } else {
        println!("ERROR")
    }
    Ok(())
}
