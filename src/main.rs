use advent_of_code_2022::day1;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Ok(input) = fs::read_to_string(&args[1]) {
        println!("{:?}", day1::calculate_calories_top_three(&input))
    } else {
        println!("ERROR")
    }
}
