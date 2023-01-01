use std::fmt::*;

use FileType::*;

#[derive(Debug, PartialEq, Copy, Clone)]
enum FileType {
    File,
    Directory,
}

#[derive(Debug, Clone)]
struct Node {
    _name: String,
    childrens: Vec<Node>,
    size: u64,
    file_type: FileType,
}

pub(crate) fn problem7_1() -> u64 {
    let input: Vec<&str> = include_str!("../inputs/7_1.txt")
        .lines()
        .collect();
    let (root, _) = parse(&input[..], "/".to_string(), 1);
    calculate_size(vec![root])
}

pub(crate) fn problem7_2() -> u64 {
    let input: Vec<&str> = include_str!("../inputs/7_1.txt")
        .lines()
        .collect();
    let (root, _) = parse(&input[..], "/".to_string(), 1);
    let total_space: u64 = 70000000;
    let space_required: u64 = 30000000;
    let space_to_free = space_required - (total_space - root.size);
    smallest_deletable_dir(vec![root], space_to_free, u64::MAX)
}

fn smallest_deletable_dir(childrens: Vec<Node>, space_to_free: u64, mut to_delete: u64) -> u64 {
    for node in childrens {
        if node.file_type == Directory && node.size >= space_to_free && node.size < to_delete {
            to_delete = node.size;
        }
        to_delete = smallest_deletable_dir(node.childrens, space_to_free, to_delete);
    }
    to_delete
}

fn calculate_size(childrens: Vec<Node>) -> u64 {
    let max_size = 100000;
    let mut size_sum = 0;
    for node in childrens {
        if node.file_type == Directory && node.size <= max_size {
            size_sum += node.size;
        }
        size_sum += calculate_size(node.childrens);
    }
    size_sum
}

fn parse(input: &[&str], name: String, mut already_read: usize) -> (Node, usize) {
    let cd = "$ cd ";
    let prev_dir = "$ cd ..";

    let mut childrens: Vec<Node> = vec![];
    let mut size = 0;
    for (index, line) in input.iter().enumerate() {
        if index > already_read {
            if line.starts_with(prev_dir) {
                return (
                    Node {
                        _name: name,
                        childrens,
                        size,
                        file_type: Directory,
                    },
                    index,
                );
            }

            if line.starts_with(cd) {
                let name = line
                    .split_at(cd.len())
                    .1
                    .to_string();
                let (node, already_read_) = parse(input, name, index);
                size += node.size;
                childrens.push(node);
                already_read = already_read_;
            } else if line.as_bytes()[0].is_ascii_digit() {
                let mut parts = line.split_whitespace();
                let file_size = parts
                    .next()
                    .map(|s| s.parse::<u64>());
                let name = parts
                    .next()
                    .map(|s| s.to_string());
                if let (Some(Ok(file_size)), Some(name)) = (file_size, name) {
                    childrens.push(Node {
                        _name: name,
                        childrens: vec![],
                        size: file_size,
                        file_type: File,
                    });

                    size += file_size;
                }
            }
        }
    }

    (
        Node {
            _name: name,
            childrens,
            size,
            file_type: Directory,
        },
        input.len(),
    )
}
