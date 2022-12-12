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

const MOVE_TAIL_DIAGONALLY: [(i32, i32); 4] = [(-2, 2), (-2, -2), (2, -2), (2, 2)];

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

#[derive(Clone, Debug, Copy)]
struct RopeObject {
    position: Position,
}

#[derive(Clone, Debug, Copy)]
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

//fn print_map(map: &Vec<Vec<bool>>) {
//println!();
//for (y, row) in map.iter().enumerate() {
//for (x, _) in row.iter().enumerate() {
//match map[y][x] {
//true => print!("#"),
//false => print!("-"),
//}
//}
//print!("\n");
//}
//}

fn do_movements(objects: &mut Vec<RopeObject>, map: &mut Vec<Vec<bool>>, instruction: &Movement) {
    for _ in 0..instruction.steps {
        match instruction.direction {
            Direction::Up => objects[0].position.y -= 1,
            Direction::Down => objects[0].position.y += 1,
            Direction::Left => objects[0].position.x -= 1,
            Direction::Right => objects[0].position.x += 1,
        }
        for i in 1..objects.len() {
            do_movement_tail(objects, i, map);
        }
    }
}

fn do_movement_tail(
    objects: &mut Vec<RopeObject>,
    current_active: usize,
    map: &mut Vec<Vec<bool>>,
) {
    let objects_len = objects.len() - 1;
    let head = objects[current_active - 1];
    let mut tail = &mut objects[current_active];

    match (
        head.position.x as i32 - tail.position.x as i32,
        head.position.y as i32 - tail.position.y as i32,
    ) {
        (x, y) if TOUCHING.contains(&(x, y)) => (), // Here we dont move at all
        (x, y) if MOVE_UP_DOWN_RIGHT_LEFT.contains(&(x, y)) => {
            if current_active == objects_len {
                map[tail.position.y as usize][tail.position.x as usize] = true;
            }
            match (x, y) {
                (-2, 0) => tail.position.x -= 1,
                (0, -2) => tail.position.y -= 1,
                (2, 0) => tail.position.x += 1,
                (0, 2) => tail.position.y += 1,
                _ => panic!("Not possibru"),
            }
        }
        (x, y) if MOVE_DIAGONALLY.contains(&(x, y)) => {
            if current_active == objects_len {
                map[tail.position.y as usize][tail.position.x as usize] = true;
            }
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
        (x, y) if MOVE_TAIL_DIAGONALLY.contains(&(x, y)) => {
            if current_active == objects_len {
                map[tail.position.y as usize][tail.position.x as usize] = true;
            }
            match (x, y) {
                (-2, 2) => {
                    tail.position.x -= 1;
                    tail.position.y += 1;
                }
                (-2, -2) => {
                    tail.position.x -= 1;
                    tail.position.y -= 1;
                }
                (2, -2) => {
                    tail.position.x += 1;
                    tail.position.y -= 1;
                }
                (2, 2) => {
                    tail.position.x += 1;
                    tail.position.y += 1;
                }
                (_, _) => panic!("Impossibruuu"),
            }
        }

        (_, _) => panic!("Head is too far away!"),
    }
}

pub fn calculate(slice: &str, number_of_knots: usize) -> u32 {
    let instructions = parse_input(slice);
    let map_size = 1000;
    let mut map: Vec<Vec<bool>> = vec![vec![false; map_size]; map_size];
    let start_point = map_size / 2;
    map[start_point][start_point] = true;

    let head = RopeObject {
        position: Position {
            x: start_point,
            y: start_point,
        },
    };

    let mut objects: Vec<RopeObject> = vec![head.clone(); number_of_knots];

    for instruction in instructions.iter() {
        do_movements(&mut objects, &mut map, instruction);
        //print_map(&map);
    }

    // Capture last position
    map[objects[number_of_knots - 1].position.y as usize]
        [objects[number_of_knots - 1].position.x] = true;

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
        assert_eq!(calculate(INPUT, 2), 13);
    }

    const INPUT_PART2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn test_part2() {
        assert_eq!(calculate(INPUT_PART2, 10), 36)
    }
}
