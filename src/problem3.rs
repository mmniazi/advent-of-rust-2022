use std::collections::HashSet;

fn score(c: u8) -> u64 {
    match c {
        p @ b'a'..=b'z' => (p - b'a' + 1) as u64,
        p @ b'A'..=b'Z' => (p - b'A' + 27) as u64,
        _ => 0,
    }
}

pub(crate) fn problem3_1() -> u64 {
    include_str!("../inputs/3_1.txt")
        .lines()
        .map(|line| {
            let size = line.len();
            let (left, right) = line.split_at(size / 2);
            let left_set: HashSet<_> = left.bytes().collect();
            let right_set: HashSet<_> = right.bytes().collect();
            let intersection: HashSet<&_> = left_set
                .intersection(&right_set)
                .collect();
            intersection
                .into_iter()
                .map(|c| score(*c))
                .sum::<u64>()
        })
        .sum()
}

pub(crate) fn problem3_2() -> u64 {
    include_str!("../inputs/3_1.txt")
        .lines()
        .array_chunks::<3>()
        .map(|[f, s, t]| {
            f.bytes()
                .find(|c| s.bytes().any(|p| &p == c) && t.bytes().any(|p| &p == c))
                .expect("a common byte between three parts")
        })
        .map(score)
        .sum()
}
