use std::collections::BinaryHeap;

fn parse() -> BinaryHeap<i64> {
    include_str!("../inputs/1_1.txt")
        .split("\n\n")
        .map(|block| {
            block
                .lines()
                .map(|line| {
                    line.trim()
                        .parse::<i64>()
                        .expect("valid number")
                })
                .sum()
        })
        .collect()
}

pub(crate) fn problem1_1() -> i64 {
    parse()
        .pop()
        .expect("to contains at least 1 line")
}

pub(crate) fn problem1_2() -> i64 {
    parse()
        .into_iter_sorted()
        .take(3)
        .sum()
}
