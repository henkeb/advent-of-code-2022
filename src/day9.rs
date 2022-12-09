use core::str::FromStr;

const TOUCHING: [(i32, i32); 9] = [
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (0, 0),
];

const MOVE_UP_DOWN_RIGHT_LEFT: [(i32, i32); 4] = [(-2, 0), (0, -2), (2, 0), (0, 2)];

const MOVE_DIAGONALLY: [(i32, i32); 8] = [
    (-2, 1),
    (-2, -1),
    (-1, -2),
    (1, -2),
    (2, -1),
    (2, 1),
    (1, 2),
    (-1, 2),
];

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(input: &str) -> Result<Direction, Self::Err> {
        match input {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Movement {
    direction: Direction,
    steps: usize,
}

#[derive(Clone, Debug)]
struct RopeObject {
    position: Position,
}

#[derive(Clone, Debug)]
struct Position {
    x: usize,
    y: usize,
}

fn parse_input(slice: &str) -> Vec<Movement> {
    slice
        .lines()
        .map(|input| {
            let split = input.split_once(" ").unwrap();
            Movement {
                direction: split.0.parse::<Direction>().unwrap(),
                steps: split.1.parse().unwrap(),
            }
        })
        .collect()
}

fn print_map(map: &Vec<Vec<bool>>) {
    println!();
    for (y, row) in map.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            match map[y][x] {
                true => print!("#"),
                false => print!("-"),
            }
        }
        print!("\n");
    }
}

fn do_movements(
    head: &mut RopeObject,
    tail: &mut RopeObject,
    map: &mut Vec<Vec<bool>>,
    instruction: &Movement,
) {
    match instruction.direction {
        Direction::Up => {
            for _ in 0..instruction.steps {
                head.position.y -= 1;
                do_movement_tail(tail, &head, map);
            }
        }
        Direction::Down => {
            for _ in 0..instruction.steps {
                head.position.y += 1;
                do_movement_tail(tail, &head, map);
            }
        }
        Direction::Left => {
            for _ in 0..instruction.steps {
                head.position.x -= 1;
                do_movement_tail(tail, &head, map);
            }
        }
        Direction::Right => {
            for _ in 0..instruction.steps {
                head.position.x += 1;
                do_movement_tail(tail, &head, map);
            }
        }
    }
}

fn do_movement_tail(tail: &mut RopeObject, head: &RopeObject, map: &mut Vec<Vec<bool>>) {
    //println!("Head: {:?}", head);
    //println!("Tail: {:?}", tail);
    //println!("Diff x: {}", head.position.x as i32 - tail.position.x as i32);
    //println!("Diff y: {}", head.position.y as i32 - tail.position.y as i32);
    match (
        head.position.x as i32 - tail.position.x as i32,
        head.position.y as i32 - tail.position.y as i32,
    ) {
        (x, y) if TOUCHING.contains(&(x, y)) => (), // Here we dont move at all
        (x, y) if MOVE_UP_DOWN_RIGHT_LEFT.contains(&(x, y)) => {
            map[tail.position.y as usize][tail.position.x as usize] = true;
            match (x, y) {
                (-2, 0) => tail.position.x -= 1,
                (0, -2) => tail.position.y -= 1,
                (2, 0) => tail.position.x += 1,
                (0, 2) => tail.position.y += 1,
                _ => panic!("Not possibru"),
            }
        }
        (x, y) if MOVE_DIAGONALLY.contains(&(x, y)) => {
            map[tail.position.y as usize][tail.position.x as usize] = true;
            match (x, y) {
                (-1, 2) | (-2, 1) => {
                    tail.position.x -= 1;
                    tail.position.y += 1
                }
                (-2, -1) | (-1, -2) => {
                    tail.position.x -= 1;
                    tail.position.y -= 1;
                }
                (1, -2) | (2, -1) => {
                    tail.position.x += 1;
                    tail.position.y -= 1;
                }
                (2, 1) | (1, 2) => {
                    tail.position.x += 1;
                    tail.position.y += 1;
                }
                _ => panic!("Impossibruuu"),
            }
        }
        (_, _) => panic!("Head is too far away!"),
    }
}

pub fn calculate(slice: &str) -> u32 {
    let instructions = parse_input(slice);
    let map_size = 1000;
    let mut map: Vec<Vec<bool>> = vec![vec![false; map_size]; map_size];
    let start_point = map_size / 2;
    map[start_point][start_point] = true;

    let mut head = RopeObject {
        position: Position {
            x: start_point,
            y: start_point,
        },
    };

    let mut tail = head.clone();

    for instruction in instructions.iter() {
        do_movements(&mut head, &mut tail, &mut map, instruction);
        //print_map(&map);
    }

    // Capture last position
    map[tail.position.y as usize][tail.position.x as usize] = true;

    let mut count = 0;

    for row in map.iter() {
        for square in row.iter() {
            if *square == true {
                count += 1;
            }
        }
    }
    count
}

pub fn calculate_part2(_slice: &str) -> u32 {
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn test_parsing() {
        assert_eq!(
            parse_input(INPUT)[7],
            Movement {
                direction: Direction::Right,
                steps: 2
            }
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(calculate(INPUT), 13);
    }

    #[test]
    fn test_part2() {}
}
