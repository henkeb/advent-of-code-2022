pub fn calculate_calories(slice: &str) -> u32 {
    let output = slice
        .trim()
        .split("\n\n")
        .map(|calories| {
            calories
                .split("\n")
                .map(|calorie| str::parse::<u32>(calorie).unwrap())
                .sum()
        })
        .max()
        .unwrap();
    output
}

pub fn calculate_calories_top_three(slice: &str) -> u32 {
    let mut n = slice
        .trim()
        .split("\n\n")
        .map(|sub| {
            sub.trim()
                .split("\n")
                .map(|num| num.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();
    n.sort_by(|a, b| b.cmp(a));
    [n[0], n[1], n[2]].iter().sum()
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
