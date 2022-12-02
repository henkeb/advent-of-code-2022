use exitfailure::ExitFailure;
use std::env;
use std::fs;

use advent_of_code_2022::day3;

fn main() -> Result<(), ExitFailure> {
    let args: Vec<String> = env::args().collect();
    if let Ok(input) = fs::read_to_string(&args[1]) {
        println!("{:?}", day3::calculate(&input).unwrap())
    } else {
        println!("ERROR")
    }
    Ok(())
}
