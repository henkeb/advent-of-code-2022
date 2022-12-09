fn parse_input(slice: &str) -> Vec<Vec<u8>> {
    slice
        .lines()
        .map(|row| {
            row.chars()
                .map(|tree| tree.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

fn check_trees_left_right(trees: &[u8], active_tree: &u8) -> bool {
    trees.iter().max().unwrap() < active_tree
}

fn check_trees_up_down(trees: &[Vec<u8>], active_tree: &u8, column: usize) -> bool {
    trees
        .iter()
        .map(|col_trees| col_trees[column])
        .max()
        .unwrap()
        < *active_tree
}

pub fn calculate(slice: &str) -> usize {
    let map = parse_input(slice);
    let mut tree_count = 0;

    tree_count += (map.len() - 2) * 2; // get rows
    tree_count += (map[0].len() - 2) * 2; // get columns
    tree_count += 4; // get corner trees

    for row in 1..map.len() - 1 {
        for col in 1..map.len() - 1 {
            let active_row = map.get(row).unwrap();
            let active_tree = active_row.get(col).unwrap();

            if check_trees_left_right(&active_row[0..col], &active_tree)
                || check_trees_left_right(&active_row[col + 1..active_row.len()], active_tree)
                || check_trees_up_down(&map[0..row], &active_tree, col)
                || check_trees_up_down(&map[row + 1..map.len()], &active_tree, col)
            {
                tree_count += 1;
            }
        }
    }

    tree_count
}

struct Position {
    row: usize,
    col: usize,
}

fn see_up(map: &Vec<Vec<u8>>, position: &Position) -> usize {
    let active_tree = map[position.row][position.col];
    (0..position.row)
        .rev()
        .take_while(|&tree_in_front| map[tree_in_front][position.col] < active_tree)
        .count()
        + 1
}

fn see_down(map: &Vec<Vec<u8>>, position: &Position) -> usize {
    let active_tree = map[position.row][position.col];
    (position.row + 1..map.len())
        .take_while(|&tree_in_front| map[tree_in_front][position.col] < active_tree)
        .count()
}

fn see_left(map: &Vec<Vec<u8>>, position: &Position) -> usize {
    let active_tree = map[position.row][position.col];
    (0..position.col)
        .rev()
        .take_while(|&tree_in_front| map[position.row][tree_in_front] < active_tree)
        .count()
}

fn see_right(map: &Vec<Vec<u8>>, position: &Position) -> usize {
    let active_tree = map[position.row][position.col];
    (position.col + 1..map[0].len())
        .take_while(|&tree_in_front| map[position.row][tree_in_front] < active_tree)
        .count()
}

fn get_scenic_score(map: &Vec<Vec<u8>>, position: Position) -> usize {
    let functions = [see_up, see_down, see_right, see_left];
    functions.iter().map(|f| f(map, &position)).product()
}

fn scenic_score(map: &Vec<Vec<u8>>) -> usize {
    let mut high_score = 0;
    for row in 1..map.len() - 1 {
        for col in 1..map[row].len() - 1 {
            let score = get_scenic_score(map, Position { row, col });
            if score > high_score {
                high_score = score;
            }
        }
    }
    high_score
}

pub fn calculate_part2(slice: &str) -> usize {
    let map = parse_input(slice);
    scenic_score(&map)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_example_input() {
        let input = "30373
25512
65332
33549
35390";
        let resulting_vec: Vec<Vec<u8>> = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];
        assert_eq!(parse_input(input), resulting_vec);
    }

    #[test]
    fn part_1() {
        let input = "30373
25512
65332
33549
35390";
        assert_eq!(21, calculate(input));
    }
}
