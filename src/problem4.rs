fn parse_elf(s: &str) -> (u32, u32) {
    let (start, end) = s
        .split_once('-')
        .expect("start and end");
    let start: u32 = start
        .parse()
        .expect("valid number");
    let end: u32 = end
        .parse()
        .expect("valid number");
    (start, end)
}

fn either_is_subset_of_other((first_start, first_end): (u32, u32), (second_start, second_end): (u32, u32)) -> bool {
    (first_start >= second_start && first_end <= second_end) || (second_start >= first_start && second_end <= first_end)
}

fn any_overlap(first @ (first_start, first_end): (u32, u32), second @ (second_start, second_end): (u32, u32)) -> bool {
    (first_end >= second_start && first_end <= second_end) || either_is_subset_of_other(first, second) || (first_start >= second_start && first_start <= second_end)
}

fn parse() -> Vec<(bool, bool)> {
    include_str!("../inputs/4_1.txt")
        .lines()
        .map(|line| {
            let (first, second) = line
                .split_once(',')
                .expect("two elves");
            let first = parse_elf(first);
            let second = parse_elf(second);
            let solution1 = either_is_subset_of_other(first, second);
            let solution2 = any_overlap(first, second);
            (solution1, solution2)
        })
        .collect()
}

pub(crate) fn problem4_1() -> usize {
    parse()
        .iter()
        .filter(|(solution1, _)| *solution1)
        .count()
}

pub(crate) fn problem4_2() -> usize {
    parse()
        .iter()
        .filter(|(_, solution2)| *solution2)
        .count()
}
