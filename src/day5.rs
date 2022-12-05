#[derive(Debug)]
struct Instruction {
    operations: usize,
    from: usize,
    to: usize,
}

impl FromIterator<usize> for Instruction {
    fn from_iter<T: IntoIterator<Item = usize>>(iter: T) -> Self {
        let mut a = iter.into_iter();
        Instruction {
            operations: a.next().unwrap(),
            from: a.next().unwrap(),
            to: a.next().unwrap(),
        }
    }
}

pub fn calculate(slice: &str, cratemover9001: bool) -> String {
    let (configuration, instruction) = slice.split_once("\n\n").unwrap();
    let mut stacks = parse_configuration(configuration);
    let instructions = parse_instruction(instruction);

    for instruction in instructions.iter() {
        move_crates(&mut stacks, instruction, cratemover9001);
    }

    stacks
        .iter()
        .filter_map(|stack| stack.last())
        .collect::<String>()
}

fn move_crates(stacks: &mut Vec<Vec<char>>, instruction: &Instruction, cratemover9001: bool) -> () {
    let moving_stack = stacks.get_mut(instruction.from - 1).unwrap();
    let mut crates = moving_stack.split_off(moving_stack.len() - instruction.operations);
    if !cratemover9001 {
        crates.reverse();
    }
    let into_stack = stacks.get_mut(instruction.to - 1).unwrap();
    into_stack.append(&mut crates);
}

fn parse_configuration(configuration: &str) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = Vec::new();
    configuration.lines().rev().skip(1).for_each(|line| {
        line.chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .filter(|(_, ch)| !ch.is_ascii_whitespace())
            .for_each(|(i, ch)| {
                if i >= stacks.len() {
                    let mut crates_vec = Vec::new();
                    crates_vec.push(ch);
                    stacks.push(crates_vec)
                } else {
                    stacks[i].push(ch);
                }
            })
    });
    stacks
}

fn parse_instruction(instruction: &str) -> Vec<Instruction> {
    let mut output: Vec<Instruction> = Vec::new();

    instruction
        .lines()
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            let instruction = line
                .split_whitespace()
                .filter_map(|word| word.parse::<usize>().ok())
                .collect::<Instruction>();
            output.push(instruction);
        });
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_the_configuration() {
        let input = "
move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2

";
        let output = parse_instruction(&input);
        assert_eq!(output[0].operations, 1);
        assert_eq!(output[0].from, 2);
        assert_eq!(output[0].to, 1);
    }

    #[test]
    fn parsing_the_instruction() {
        let input = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3";
        assert_eq!(
            parse_configuration(&input),
            vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']]
        );
    }
    #[test]
    fn part1() {
        let input = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2

";
        assert_eq!(calculate(&input, false), "CMZ");
    }

    #[test]
    fn part2() {
        let input = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2

";
        assert_eq!(calculate(&input, true), "MCD");
    }
}
