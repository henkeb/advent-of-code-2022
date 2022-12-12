use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Ok(input) = fs::read_to_string(&args[1]) {
        match args[1].split(&['/', '.']).nth(1).unwrap() {
            "day1" => println!(
                "Part1: {}\nPart2: {}",
                advent_of_code_2022::day1::calculate_calories(&input),
                advent_of_code_2022::day1::calculate_calories_top_three(&input)
            ),
            "day2" => println!(
                "(Part1, Part2): {:?}",
                advent_of_code_2022::day2::calculate(&input).unwrap(),
            ),
            "day3" => println!(
                "Part1: {}\nPart2: {}",
                advent_of_code_2022::day3::calculate(&input),
                advent_of_code_2022::day3::calculate_part2(&input)
            ),
            "day4" => println!(
                "Part1: {}\nPart2: {}",
                advent_of_code_2022::day4::calculate(&input),
                advent_of_code_2022::day4::calculate_2(&input)
            ),
            "day5" => println!(
                "Part1: {}\nPart2: {}",
                advent_of_code_2022::day5::calculate(&input, false),
                advent_of_code_2022::day5::calculate(&input, true)
            ),
            "day6" => println!(
                "Part1: {}\nPart2: {}",
                advent_of_code_2022::day6::calculate(&input, 4).unwrap(),
                advent_of_code_2022::day6::calculate(&input, 14).unwrap()
            ),
            "day7" => println!(
                "Part1: {}\nPart2: {}",
                advent_of_code_2022::day7::calculate(&input),
                advent_of_code_2022::day7::calculate_part2(&input)
            ),
            "day8" => {
                println!(
                    "Part1: {}\nPart2: {}",
                    advent_of_code_2022::day8::calculate(&input),
                    advent_of_code_2022::day8::calculate_part2(&input)
                )
            }
            "day9" => {
                println!(
                    "Part1: {}\nPart2: {}",
                    advent_of_code_2022::day9::calculate(&input, 2),
                    advent_of_code_2022::day9::calculate(&input, 10),
                )
            }
            "day10" => {
                println!("Part1: {}\n", advent_of_code_2022::day10::calculate(&input),);
                advent_of_code_2022::day10::calculate_part2(&input);
            }
            _ => (),
        }
    } else {
        println!("Could not read input file");
    }
}
