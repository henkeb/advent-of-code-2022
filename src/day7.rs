use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

struct Tree<'a> {
    root: Rc<RefCell<Node<'a>>>,
}

struct Node<'a> {
    name: &'a str,
    parent: Option<Weak<RefCell<Node<'a>>>>,
    directories: Vec<Rc<RefCell<Node<'a>>>>,
    files: Vec<File>,
}

struct File {
    size: usize,
}

impl<'a> Tree<'a> {
    fn get_size(&self, node: Option<Rc<RefCell<Node<'a>>>>, result: &mut Vec<usize>) -> usize {
        let node = node.unwrap_or_else(|| self.root.clone());
        let mut dir_sum = 0;
        node.borrow().directories.iter().for_each(|dir| {
            dir_sum += self.get_size(Some(dir.clone()), result);
        });
        let file_sum: usize = node.borrow().files.iter().map(|file| file.size).sum();
        result.push(file_sum + dir_sum);
        file_sum + dir_sum
    }
}

fn parse_input(input: &str) -> Tree {
    let tree = Tree {
        root: Rc::new(RefCell::new(Node {
            name: "/",
            parent: None,
            directories: vec![],
            files: vec![],
        })),
    };

    let mut active_node = tree.root.clone();

    input
        .lines()
        .skip(1)
        .for_each(|line| match line.split_whitespace().peekable().peek() {
            Some(&"$") => do_command(line.clone(), &mut active_node),
            Some(&"dir") => {
                let dir_name = line.split_whitespace().last();
                active_node
                    .borrow_mut()
                    .directories
                    .push(Rc::new(RefCell::new(Node {
                        name: dir_name.unwrap(),
                        parent: Some(Rc::downgrade(&active_node)),
                        directories: vec![],
                        files: vec![],
                    })))
            }
            _ => {
                let split_line = line.split_once(" ");
                let size = split_line.unwrap().0;
                active_node.borrow_mut().files.push(File {
                    size: size.parse::<usize>().unwrap(),
                })
            }
        });
    tree
}
fn do_command(command: &str, curr_node: &mut Rc<RefCell<Node>>) -> ()
where
{
    match command.clone().split_whitespace().skip(1).next() {
        Some("cd") => {
            let name = command.split_whitespace().last();
            if name == Some("..") {
                let temp_node = curr_node.borrow().parent.as_ref().unwrap().clone();
                *curr_node = temp_node.upgrade().unwrap();
            } else {
                let temp_node = curr_node
                    .borrow()
                    .directories
                    .iter()
                    .find(|node| node.borrow().name == name.unwrap())
                    .unwrap()
                    .clone();
                *curr_node = temp_node;
            }
        }
        Some(_) => (),
        None => (),
    }
}

pub fn calculate(slice: &str) -> usize {
    let tree = parse_input(slice);
    let mut result: Vec<usize> = vec![];
    tree.get_size(None, &mut result);
    result
        .iter()
        .filter(|&&size| size <= 100_000 as usize)
        .sum()
}
pub fn calculate_part2(slice: &str) -> usize {
    let tree = parse_input(slice);
    let mut result: Vec<usize> = vec![];
    let total_size = tree.get_size(None, &mut result);
    let unused_space = 70000000 - total_size;

    result
        .into_iter()
        .filter(|&size| unused_space + size >= 30000000)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

        assert_eq!(95437, calculate(&input));
    }

    #[test]
    fn test_part2() {
        let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

        assert_eq!(24933642, calculate_part2(&input));
    }
}
