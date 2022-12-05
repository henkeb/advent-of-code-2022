use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
enum Sign {
    Rock,
    Paper,
    Scissors,
}

enum MatchResults {
    Win,
    Draw,
    Loss,
}

impl MatchResults {
    fn value(&self) -> i32 {
        match *self {
            MatchResults::Win => 6,
            MatchResults::Draw => 3,
            MatchResults::Loss => 0,
        }
    }
}

impl Sign {
    fn value(&self) -> i32 {
        match *self {
            Sign::Rock => 1,
            Sign::Paper => 2,
            Sign::Scissors => 3,
        }
    }
}

impl FromStr for Sign {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Sign::Rock),
            "B" | "Y" => Ok(Sign::Paper),
            "C" | "Z" => Ok(Sign::Scissors),
            _ => Err(()),
        }
    }
}

pub fn calculate(slice: &str) -> Result<(i32, i32), ()> {
    Ok(slice
        .trim()
        .lines()
        .map(|round| round.split_once(" ").unwrap())
        .fold((0, 0), |(acc_part1, acc_part2), (elf, you)| {
            let elf_parsed = elf.parse::<Sign>().unwrap();
            let you_parsed = you.parse::<Sign>().unwrap();
            (
                acc_part1 + points_per_round(&elf_parsed, &you_parsed),
                acc_part2 + points_per_round_part2(&elf_parsed, &you_parsed),
            )
        }))
}

fn points_per_round(elf: &Sign, you: &Sign) -> i32 {
    let point = match (elf, you) {
        (Sign::Rock, Sign::Paper) => MatchResults::Win.value(),
        (Sign::Rock, Sign::Scissors) => MatchResults::Loss.value(),
        (Sign::Paper, Sign::Rock) => MatchResults::Loss.value(),
        (Sign::Paper, Sign::Scissors) => MatchResults::Win.value(),
        (Sign::Scissors, Sign::Rock) => MatchResults::Win.value(),
        (Sign::Scissors, Sign::Paper) => MatchResults::Loss.value(),
        (Sign::Rock, Sign::Rock) => MatchResults::Draw.value(),
        (Sign::Paper, Sign::Paper) => MatchResults::Draw.value(),
        (Sign::Scissors, Sign::Scissors) => MatchResults::Draw.value(),
    };
    point + you.value()
}

fn points_per_round_part2(elf: &Sign, you: &Sign) -> i32 {
    match (elf, you) {
        (Sign::Rock, Sign::Rock) => MatchResults::Loss.value() + Sign::Scissors.value(),
        (Sign::Paper, Sign::Rock) => MatchResults::Loss.value() + Sign::Rock.value(),
        (Sign::Scissors, Sign::Rock) => MatchResults::Loss.value() + Sign::Paper.value(),
        (elf, Sign::Paper) => MatchResults::Draw.value() + elf.value(),
        (Sign::Rock, Sign::Scissors) => MatchResults::Win.value() + Sign::Paper.value(),
        (Sign::Paper, Sign::Scissors) => MatchResults::Win.value() + Sign::Scissors.value(),
        (Sign::Scissors, Sign::Scissors) => MatchResults::Win.value() + Sign::Rock.value(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_point() {
        let input: &str = "A Y
B X
C Z";
        assert_eq!(calculate(&input), Ok((15, 12)));
    }

    #[test]
    fn calculate_rock_win() {
        assert_eq!(points_per_round(&Sign::Scissors, &Sign::Rock), 7);
    }
}

// First implementation of part 1 below
//pub fn calculate(slice: &str) -> Result<i32, ()> {
//slice
//.trim()
//.split("\n")
//.map(|round| round.split(" ").collect::<Vec<&str>>())
//.map(|line| points_per_round(&Sign::from_str(line[0])?, &Sign::from_str(line[1])?))
//.sum()
//}
