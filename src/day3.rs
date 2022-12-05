// Rewritten into functional style which i want to learn. HEAVILY inspired by
// https://github.com/markus-k/adventofcode/blob/main/2022/day03/src/main.rs

pub fn calculate(slice: &str) -> i32 {
    slice
        .lines()
        .filter_map(|line| common_chars(split_rucksack(line)))
        .map(get_character_value)
        .sum()
}

fn split_rucksack(rucksack: &str) -> (&str, &str) {
    rucksack.split_at(rucksack.len() / 2)
}

fn common_chars(rucksack_sides: (&str, &str)) -> Option<char> {
    rucksack_sides
        .0
        .chars()
        .find(|ch| rucksack_sides.1.contains(*ch))
}

fn get_character_value(ch: char) -> i32 {
    match ch {
        small if small.is_lowercase() => small as i32 - 96,
        capital if capital.is_uppercase() => capital as i32 - 65 + 27,
        _ => 0,
    }
}

pub fn calculate_part2(slice: &str) -> i32 {
    slice
        .lines()
        .collect::<Vec<_>>()
        .chunks_exact(3)
        .filter_map(find_common_char)
        .map(get_character_value)
        .sum()
}

fn find_common_char(lines: &[&str]) -> Option<char> {
    lines
        .first()?
        .chars()
        .find(|ch| lines[1..].iter().all(|shared| shared.contains(*ch)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_priority() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(calculate(&input), 157);
    }

    #[test]
    fn calculate_priority_part2() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(calculate_part2(&input), 70);
    }
}
