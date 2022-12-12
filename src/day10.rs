use core::str::FromStr;

#[derive(Debug, PartialEq)]
enum Operation {
    Noop,
    Addx,
}

#[derive(Debug, PartialEq)]
struct Mnemonic {
    operation: Operation,
    cycles: u8,
}

impl FromStr for Mnemonic {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "noop" => Ok(Mnemonic {
                operation: Operation::Noop,
                cycles: 1,
            }),
            "addx" => Ok(Mnemonic {
                operation: Operation::Addx,
                cycles: 2,
            }),
            _ => Err(()),
        }
    }
}

fn parse_input(slice: &str) -> Vec<(Mnemonic, Option<i32>)> {
    slice
        .lines()
        .map(|line| match line.split_once(" ") {
            Some(mnemonic_value) => (
                mnemonic_value.0.parse::<Mnemonic>().unwrap(),
                Some(mnemonic_value.1.parse::<i32>().unwrap()),
            ),
            None => (line.parse::<Mnemonic>().unwrap(), None),
        })
        .collect::<Vec<(Mnemonic, Option<i32>)>>()
}

pub fn calculate(slice: &str) -> i32 {
    let instructions = parse_input(slice);
    let mut clock = 0;
    let mut register_x = 1;
    let mut sum_cycle = 0;
    let mut clock_frequency = 20;

    for instruction in instructions.iter() {
        for _ in 0..instruction.0.cycles {
            clock += 1;
            if clock == clock_frequency {
                sum_cycle += register_x * clock;
                clock_frequency += 40;
            }
        }
        match instruction.0.operation {
            Operation::Addx => register_x += instruction.1.unwrap(),
            Operation::Noop => (),
        }
    }
    sum_cycle
}

pub fn calculate_part2(slice: &str) {
    let instructions = parse_input(slice);
    let mut clock = 0;
    let mut register_x = 1;
    let mut sprite_clock = 0;

    for instruction in instructions.iter() {
        for _ in 0..instruction.0.cycles {
            if (register_x - 1..=register_x + 1).contains(&sprite_clock) {
                print!("#");
            } else {
                print!(".")
            }
            if sprite_clock == 40 {
                print!("\n");
                sprite_clock = 1;
            } else {
                sprite_clock += 1;
            }
            clock += 1;
        }
        match instruction.0.operation {
            Operation::Addx => register_x += instruction.1.unwrap(),
            Operation::Noop => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "noop
addx 3
addx -5";

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input(INPUT)[0].0,
            Mnemonic {
                operation: Operation::Noop,
                cycles: 1
            }
        )
    }
    const INPUT1: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn test_part1() {
        assert_eq!(calculate(INPUT1), 13140);
    }

    #[test]
    fn test_part2() {
        calculate_part2(INPUT1);
    }
}
