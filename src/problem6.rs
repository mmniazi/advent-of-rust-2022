use std::collections::VecDeque;

use itertools::Itertools;

pub(crate) fn problem6(is_part_two: bool) -> usize {
    let unique_character_count = if is_part_two { 14 } else { 4 };
    let mut queue: VecDeque<char> = VecDeque::new();
    let input = include_str!("../inputs/6_1.txt")
        .chars()
        .enumerate();
    for (index, c) in input {
        if queue.len() == unique_character_count {
            queue.pop_front();
        }
        queue.push_back(c);
        if queue.len() == unique_character_count && queue.iter().all_unique() {
            return index + 1;
        }
    }
    0
}
