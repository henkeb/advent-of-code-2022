use std::ops::RangeInclusive;

fn is_inclusive(first: &RangeInclusive<u32>, second: &RangeInclusive<u32>) -> bool {
    (first.start() <= second.start() && second.end() <= first.end())
        || (second.start() <= first.start() && first.end() <= second.end())
}

fn is_overlapping(first: &RangeInclusive<u32>, second: &RangeInclusive<u32>) -> bool {
    ((first.start() >= second.start() && first.end() <= second.end())
        || (first.end() >= second.start() && first.end() <= second.end()))
        || ((second.start() >= first.start() && second.end() <= first.end())
            || (second.end() >= first.start() && second.end() <= first.end()))
}

pub fn calculate(slice: &str) -> usize {
    parse_input(slice)
        .filter(|(first, second)| is_inclusive(first, second))
        .count()
}

pub fn calculate_2(slice: &str) -> usize {
    parse_input(slice)
        .filter(|(first, second)| is_overlapping(first, second))
        .count()
}

fn parse_input<'slice_lifetime>(
    slice: &'slice_lifetime str,
) -> impl Iterator<Item = (RangeInclusive<u32>, RangeInclusive<u32>)> + 'slice_lifetime {
    slice
        .lines()
        .map(|line| line.split(&['-', ',']))
        .filter_map(|mut ranges| {
            Some((
                ranges.next()?.parse::<u32>().ok()?..=ranges.next()?.parse::<u32>().ok()?,
                ranges.next()?.parse::<u32>().ok()?..=ranges.next()?.parse::<u32>().ok()?,
            ))
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_part1() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";
        assert_eq!(calculate(&input), 2);
    }

    #[test]
    fn calculate_part2() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";
        assert_eq!(calculate_2(&input), 4);
    }
}
