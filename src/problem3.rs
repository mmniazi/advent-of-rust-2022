use fs::read_to_string;
use std::collections::HashSet;
use std::fs;

fn score(c: u8) -> u64 {
    match c {
        p @ b'a'..=b'z' => (p - b'a' + 1) as u64,
        p @ b'A'..=b'Z' => (p - b'A' + 27) as u64,
        _ => 0,
    }
}

pub(crate) fn problem3_1() -> u64 {
    read_to_string("inputs/3_1.txt").expect("file to be readable")
        .lines()
        .map(|line| {
            let size = line.bytes().count();
            let (left, right) = line.split_at(size / 2);
            let left_set: HashSet<_> = left.bytes().collect();
            let right_set: HashSet<_> = right.bytes().collect();
            let intersection: HashSet<&_> = left_set.intersection(&right_set).collect();
            intersection.into_iter().map(|c| score(*c)).sum::<u64>()
        })
        .sum()
}

pub(crate) fn problem3_2() -> u64 {
    read_to_string("inputs/3_1.txt").expect("file to be readable")
        .lines()
        .array_chunks::<3>()
        .map(|[f, s, t]|
            f.bytes()
                .find(|c| s.bytes().find(|p| p == c).is_some() && t.bytes().find(|p| p == c).is_some())
                .expect("a common byte between three parts")
        )
        .map(|c| score(c as u8))
        .sum()
}