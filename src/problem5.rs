use itertools::{Chunk, Itertools};
use mona::Transpose;
use regex::Regex;

pub(crate) fn problem5(is_part_two: bool) -> String {
    let input = include_str!("../inputs/5_1.txt");

    let mut boxes: Vec<Vec<char>> = input
        .lines()
        .filter(|s: &&str| s.contains('['))
        .map(|s: &str| {
            s.chars()
                .chunks(4)
                .into_iter()
                .map(|mut s: Chunk<_>| s.nth(1).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
        .transpose()
        .unwrap()
        .into_iter()
        .map(|column| {
            let mut column = column
                .into_iter()
                .filter(|c| c.is_alphabetic())
                .collect::<Vec<_>>();
            column.reverse();
            column
        })
        .collect::<Vec<_>>();

    let re = Regex::new(r"^move (\d*) from (\d*) to (\d*)$").unwrap();
    for line in input
        .lines()
        .filter(|s: &&str| s.contains("move"))
    {
        if let Some(cap) = re.captures_iter(line).next() {
            let (amount, from, to) = (
                cap[1]
                    .parse::<usize>()
                    .unwrap(),
                cap[2]
                    .parse::<usize>()
                    .unwrap()
                    - 1,
                cap[3]
                    .parse::<usize>()
                    .unwrap()
                    - 1,
            );
            if is_part_two {
                // move boxes together
                let new_size = boxes[from].len() - amount;
                let drained = boxes[from]
                    .drain(new_size..)
                    .collect::<Vec<_>>();
                boxes[to].extend(drained);
            } else {
                for _ in 0..amount {
                    let top_box = boxes[from].pop().unwrap();
                    boxes[to].push(top_box);
                }
            }
        }
    }

    boxes
        .into_iter()
        .filter_map(|column| {
            column
                .into_iter()
                .last()
                .or(Some(' '))
        })
        .collect()
}
