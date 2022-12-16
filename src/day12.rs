#![allow(dead_code)]

use std::fmt::Display;

type Coord = (usize, usize);

struct MapData {
    map: Vec<Vec<u8>>,
    start: Coord,
    end: Coord,
}

fn char_to_height(c: char) -> u8 {
    match c {
        'a'..='z' => c as u8 - b'a',
        'S' => 0,
        'E' => 25,
        _ => unreachable!(),
    }
}

fn parse_input(slice: &str) -> MapData {
    let map: Vec<Vec<u8>> = slice
        .lines()
        .map(|line| line.chars().map(char_to_height).collect::<Vec<u8>>())
        .collect();

    let find_in_map = |ch: char| -> Option<Coord> {
        slice.lines().enumerate().find_map(|(x, row)| {
            row.chars()
                .enumerate()
                .find(|&(_y, height)| height == ch)
                .map(|(y, _height)| (x, y))
        })
    };

    let start = find_in_map('S');
    let end = find_in_map('E');

    MapData {
        map,
        start: start.expect("No starting position found!"),
        end: end.expect("No end position found!"),
    }
}

fn print_map<T>(map: &[Vec<T>])
where
    T: Display,
{
    for (i, row) in map.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            print!("{}", map[i][j]);
            print!(" ");
        }
        println!();
    }
}

fn neighbour_calculation(map: &[Vec<u8>], (x, y): &Coord) -> Vec<Coord> {
    let bound_check_calculation =
        |(dx, dy): (isize, isize)| x.checked_add_signed(dx).zip(y.checked_add_signed(dy));

    let height_check = |height| (0..=(map[*x][*y] + 1)).contains(height);

    let is_valid = |&(dx, dy): &Coord| {
        map.get(dx)
            .and_then(|row| row.get(dy))
            .map_or(false, height_check)
    };

    [(-1, 0), (0, 1), (0, -1), (1, 0)]
        .into_iter()
        .filter_map(bound_check_calculation)
        .filter(is_valid)
        .collect::<Vec<Coord>>()
}

fn bfs(map: &MapData) -> Option<usize> {
    let mut visited = vec![vec![false; map.map[0].len()]; map.map.len()];

    // Push start on the queue!
    let mut queue = std::collections::VecDeque::from([(map.start, 0)]);

    while let Some(((x, y), steps)) = queue.pop_front() {
        if (x, y) == map.end {
            return Some(steps);
        }

        if visited[x][y] {
            continue;
        }

        visited[x][y] = true;

        neighbour_calculation(&map.map, &(x, y))
            .iter()
            .for_each(|neighbour| queue.push_back((*neighbour, steps + 1)));
    }
    None
}

pub fn calculate(slice: &str) -> Option<usize> {
    bfs(&parse_input(slice))
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn parse() {
        assert_eq!(parse_input(INPUT).end, (2, 5));
        assert_eq!(parse_input(INPUT).start, (0, 0));
    }

    #[test]
    fn starting_position() {
        assert_eq!(bfs(&parse_input(INPUT)), Some(31));
    }
}
