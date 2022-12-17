#![allow(dead_code)]

use std::collections::HashSet;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn parse(s: &str) -> Self {
        let (x, y) = s.trim().split_once(',').unwrap();
        Point {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
struct Line {
    points: Vec<Point>,
}

impl Line {
    fn parse(s: &str) -> Self {
        Self {
            points: s.split("->").map(Point::parse).collect(),
        }
    }

    fn get_all_points(&self) -> HashSet<Point> {
        let mut points: HashSet<Point> = HashSet::new();
        self.points.windows(2).for_each(|window| {
            let p1 = window[0];
            let p2 = window[1];
            for x in p1.x.min(p2.x)..=p1.x.max(p2.x) {
                for y in p1.y.min(p2.y)..=p1.y.max(p2.y) {
                    points.insert(Point { x, y });
                }
            }
        });
        points
    }
}

fn parse_input(input: &str) -> HashSet<Point> {
    let mut rocks = HashSet::new();
    input
        .lines()
        .map(Line::parse)
        .map(|line| line.get_all_points())
        .for_each(|rock_set| rocks.extend(rock_set));
    rocks
}

fn drop_sand(
    behind_waterfall: &mut HashSet<Point>,
    x: u32,
    y: u32,
    floor: u32,
    infinite_floor: bool,
) -> bool {
    if y == floor {
        return true;
    }

    if behind_waterfall.contains(&Point { x, y }) {
        return false;
    }

    for square in [x, x - 1, x + 1].iter() {
        if drop_sand(behind_waterfall, *square, y + 1, floor, infinite_floor) && infinite_floor {
            return true;
        }
    }

    behind_waterfall.insert(Point { x, y });
    false
}

pub fn calculate(input: &str, infinite_floor: bool) -> usize {
    let mut behind_waterfall = parse_input(input);
    let behind_waterfall_rocks = behind_waterfall.len();
    let max_y = behind_waterfall
        .iter()
        .map(|point| point.y)
        .max()
        .expect("Rocks are needed");

    drop_sand(&mut behind_waterfall, 500, 0, max_y + 2, infinite_floor);

    behind_waterfall.len() - behind_waterfall_rocks
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn check_example() {
        assert_eq!(calculate(INPUT, true), 24);
    }

    #[test]
    fn finite_floor() {
        assert_eq!(calculate(INPUT, false), 93)
    }
}
