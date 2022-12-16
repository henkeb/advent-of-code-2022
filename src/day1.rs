pub fn calculate_calories(slice: &str) -> u32 {
    slice
        .trim()
        .split("\n\n")
        .map(|calories| {
            calories
                .split('\n')
                .map(|calorie| str::parse::<u32>(calorie).unwrap())
                .sum()
        })
        .max()
        .unwrap()
}

pub fn calculate_calories_top_three(slice: &str) -> u32 {
    let mut output = slice
        .trim()
        .split("\n\n")
        .map(|calories| {
            calories
                .split('\n')
                .map(|calorie| str::parse::<u32>(calorie).unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();
    output.sort();
    output.iter().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calories() {
        let input = String::from(
            "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000",
        );
        assert_eq!(calculate_calories(&input), 24000);
    }

    #[test]
    fn test_calories_top_three() {
        let input = String::from(
            "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000",
        );
        assert_eq!(calculate_calories_top_three(&input), 45000);
    }
}
