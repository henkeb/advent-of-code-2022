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

pub fn calculate_vector(slice: &str, message_length: usize) -> Option<usize> {
    slice
        .as_bytes()
        .windows(message_length)
        .position(|w| {
            let mut values = Vec::with_capacity(message_length);
            for character in w {
                if values.contains(character) {
                    return false;
                }
                values.push(*character);
            }
            true
        })
        .map(|position| position + message_length)
}

pub fn calculate_bit_no(slice: &str, message_length: usize) -> Option<usize> {
    let mut filter = 0u32;
    slice
        .as_bytes()
        .iter()
        .take(message_length - 1)
        .for_each(|character| filter ^= 1 << (character % 32));

    eprintln!("filter befor the loop = {:b}", filter);

    slice
        .as_bytes()
        .windows(message_length)
        .position(|w| {
            let first = w[0];
            let last = w[w.len() - 1];
            filter ^= 1 << (last % 32);
            eprintln!("filter after last xor = {:b}", filter);
            let res = filter.count_ones() == message_length as u32;
            filter ^= 1 << (first % 32);
            eprintln!("filter after resetting to the first xor = {:b}", filter);
            println!();
            res
        })
        .map(|res| res + message_length)
}

pub fn calculate_bit(slice: &str, message_length: usize, start_pos: usize) -> Option<usize> {
    let mut filter = 0u32;
    slice
        .as_bytes()
        .iter()
        .for_each(|character| filter ^= 1 << (character % 32));

    if filter.count_ones() == message_length as u32 {
        Some(start_pos + message_length)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_1() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(calculate(input, 4).unwrap(), 5);
        assert_eq!(calculate_vector(input, 4).unwrap(), 5);
    }

    #[test]
    fn test_part1_2() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(calculate(input, 4).unwrap(), 6);
        assert_eq!(calculate_vector(input, 4).unwrap(), 6);
    }

    #[test]
    fn test_part1_3() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(calculate(input, 4).unwrap(), 10);
        assert_eq!(calculate_vector(input, 4).unwrap(), 10);
    }

    #[test]
    fn test_part1_4() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(calculate(input, 4).unwrap(), 11);
        assert_eq!(calculate_vector(input, 4).unwrap(), 11);
    }

    #[test]
    fn test_part2_1() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(calculate(input, 14).unwrap(), 23);
    }

    #[test]
    fn test_part2_2() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(calculate(input, 14).unwrap(), 23);
    }

    #[test]
    fn test_part2_3() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(calculate(input, 14).unwrap(), 29);
    }

    #[test]
    fn test_part2_4() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(calculate(input, 14).unwrap(), 26);
    }
}
