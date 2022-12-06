use std::collections::HashSet;

pub fn calculate(slice: &str, message_length: usize) -> Option<usize> {
    slice
        .as_bytes()
        .windows(message_length)
        .enumerate()
        .find(|(_i, window)| {
            let mut seen: HashSet<u8> = HashSet::new();
            window.iter().for_each(|ch| {
                seen.insert(*ch);
            });
            seen.len() == message_length
        })
        .map(|(i, _)| i + message_length)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_1() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(calculate(&input, 4).unwrap(), 5);
    }

    #[test]
    fn test_part1_2() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(calculate(&input, 4).unwrap(), 6);
    }

    #[test]
    fn test_part1_3() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(calculate(&input, 4).unwrap(), 10);
    }

    #[test]
    fn test_part1_4() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(calculate(&input, 4).unwrap(), 11);
    }

    #[test]
    fn test_part2_1() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(calculate(&input, 14).unwrap(), 23);
    }

    #[test]
    fn test_part2_2() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(calculate(&input, 14).unwrap(), 23);
    }

    #[test]
    fn test_part2_3() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(calculate(&input, 14).unwrap(), 29);
    }

    #[test]
    fn test_part2_4() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(calculate(&input, 14).unwrap(), 26);
    }
}
