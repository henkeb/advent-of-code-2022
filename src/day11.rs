use std::{
    collections::VecDeque,
    ops::{Add, Mul},
};

#[derive(Debug)]
struct Rule {
    divisor: usize,
    success: usize,
    fail: usize,
}

#[derive(Debug)]
struct Monkey {
    inspections: usize,
    items: VecDeque<usize>,
    worry_calculation: fn(usize, usize) -> usize,
    worry_calculator_last_operator: Option<usize>,
    item_rule: Rule,
}

impl Monkey {
    fn worry_calculation(&self, old: usize) -> usize {
        return (self.worry_calculation)(old, self.worry_calculator_last_operator.unwrap_or(old));
    }
}

fn parse_input(slice: &str) -> Vec<Monkey> {
    slice
        .split("\n\n")
        .map(|monkey| {
            let lines: Vec<&str> = monkey.lines().collect();

            let items: VecDeque<usize> = lines[1]
                .split(":")
                .nth(1)
                .unwrap()
                .split(",")
                .map(|item| item.trim().parse().unwrap())
                .collect();

            let operation: Vec<&str> = lines[2]
                .split("=")
                .nth(1)
                .unwrap()
                .trim()
                .split(" ")
                .collect();

            let divisor: usize = lines[3].split_whitespace().last().unwrap().parse().unwrap();

            let success: usize = lines[4].split_whitespace().last().unwrap().parse().unwrap();

            let fail: usize = lines[5].split_whitespace().last().unwrap().parse().unwrap();

            Monkey {
                inspections: 0,
                items,
                worry_calculation: if operation[1] == "+" {
                    Add::add
                } else {
                    Mul::mul
                },
                worry_calculator_last_operator: operation.last().unwrap().parse().ok(),
                item_rule: Rule {
                    divisor,
                    success,
                    fail,
                },
            }
        })
        .collect::<Vec<Monkey>>()
}

fn monkey_business(monkeys: &mut Vec<Monkey>, rounds: usize, worry: impl Fn(usize) -> usize) {
    for _ in 0..rounds {
        for monkey in 0..monkeys.len() {
            while let Some(mut item) = monkeys[monkey].items.pop_front() {
                monkeys[monkey].inspections += 1;

                item = monkeys[monkey].worry_calculation(item);
                item = worry(item);

                let throw_at = if item % monkeys[monkey].item_rule.divisor == 0 {
                    monkeys[monkey].item_rule.success
                } else {
                    monkeys[monkey].item_rule.fail
                };

                monkeys[throw_at].items.push_back(item);
            }
        }
    }
}

fn get_score(monkeys: Vec<Monkey>) -> usize {
    let mut inspections = monkeys
        .into_iter()
        .map(|monkey| monkey.inspections)
        .collect::<Vec<usize>>();

    inspections.sort();
    inspections.iter().rev().take(2).product()
}

pub fn calculate(slice: &str, rounds: usize) -> usize {
    let mut monkeys: Vec<Monkey> = parse_input(slice);
    monkey_business(&mut monkeys, rounds, |x| x / 3);
    get_score(monkeys)
}

pub fn calculate_part2(slice: &str, rounds: usize) -> usize {
    let mut monkeys: Vec<Monkey> = parse_input(slice);
    let lcd: usize = monkeys
        .iter()
        .map(|monkey| monkey.item_rule.divisor)
        .product();

    monkey_business(&mut monkeys, rounds, |x| x % lcd);
    get_score(monkeys)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test_parse_input() {
        parse_input(INPUT);
        assert_eq!(parse_input(INPUT)[3].items, vec![74]);
    }

    #[test]
    fn calculate_part1() {
        assert_eq!(calculate(INPUT, 20), 10605);
    }
}
