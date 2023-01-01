use std::cmp::max;

use itertools::FoldWhile::*;
use itertools::Itertools;

pub(crate) fn problem8_1() -> u32 {
    let rows: Vec<Vec<u32>> = include_str!("../inputs/8_1.txt")
        .lines()
        .map(to_digits)
        .collect();
    let mut total_visible: u32 = 0;
    for (row_index, columns) in rows.iter().enumerate() {
        for (column_index, n) in columns.iter().enumerate() {
            if row_index > 0 && row_index < rows.len() - 1 && column_index > 0 && column_index < columns.len() - 1 {
                let right_visible = rows[row_index][column_index + 1..columns.len()]
                    .iter()
                    .all(|i| n > i);
                let left_visible = rows[row_index][0..column_index]
                    .iter()
                    .all(|i| n > i);
                let top_visible = rows[0..row_index]
                    .iter()
                    .all(|row| *n > row[column_index]);
                let bottom_visible = rows[row_index + 1..rows.len()]
                    .iter()
                    .all(|row| *n > row[column_index]);

                if right_visible || left_visible || top_visible || bottom_visible {
                    total_visible += 1;
                }
            } else {
                total_visible += 1
            }
        }
    }
    total_visible
}

pub(crate) fn problem8_2() -> u32 {
    let rows: Vec<Vec<u32>> = include_str!("../inputs/8_1.txt")
        .lines()
        .map(to_digits)
        .collect();

    let mut max_scenic_score: u32 = 0;
    for (row_index, columns) in rows.iter().enumerate() {
        for (column_index, n) in columns.iter().enumerate() {
            if row_index > 0 && row_index < rows.len() - 1 && column_index > 0 && column_index < columns.len() - 1 {
                let right_visible = rows[row_index][column_index + 1..columns.len()]
                    .iter()
                    .fold_while(0, |acc, i| if n > i { Continue(acc + 1) } else { Done(acc + 1) })
                    .into_inner();
                let left_visible = rows[row_index][0..column_index]
                    .iter()
                    .rev()
                    .fold_while(0, |acc, i| if n > i { Continue(acc + 1) } else { Done(acc + 1) })
                    .into_inner();
                let top_visible = rows[0..row_index]
                    .iter()
                    .rev()
                    .fold_while(0, |acc, row| if *n > row[column_index] { Continue(acc + 1) } else { Done(acc + 1) })
                    .into_inner();
                let bottom_visible = rows[row_index + 1..rows.len()]
                    .iter()
                    .fold_while(0, |acc, row| if *n > row[column_index] { Continue(acc + 1) } else { Done(acc + 1) })
                    .into_inner();

                let scenic_score = right_visible * left_visible * top_visible * bottom_visible;
                max_scenic_score = max(scenic_score, max_scenic_score);
            }
        }
    }
    max_scenic_score
}

fn to_digits(s: &str) -> Vec<u32> {
    s.chars()
        .map(|c| {
            c.to_digit(10)
                .expect("to be digit")
        })
        .collect()
}
