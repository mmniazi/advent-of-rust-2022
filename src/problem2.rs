use fs::read_to_string;
use std::fs;

use MatchResult::*;
use Shape::*;

#[derive(PartialEq)]
enum MatchResult {
    Win,
    Draw,
    Lose,
}

impl MatchResult {
    fn score(&self) -> i64 {
        match *self {
            Win => 6,
            Draw => 3,
            Lose => 0,
        }
    }
}

#[derive(PartialEq)]
#[derive(Copy, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn score(&self) -> i64 {
        match *self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    fn defeats(&self) -> Shape {
        match *self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }

    fn defeated_by(&self) -> Shape {
        match *self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }
}

fn parse() -> Vec<(Shape, char)> {
    let file_path = "inputs/2_1.txt";

    read_to_string(file_path).expect("file to be readable")
        .lines()
        .map(|line| {
            let left_char = line.chars().nth(0).expect("left move");
            let right_char = line.chars().nth(2).expect("right move");
            let left = match left_char {
                'A' => Some(Rock),
                'B' => Some(Paper),
                'C' => Some(Scissors),
                _ => None
            }.expect("value to be A, B or C");
            (left, right_char)
        })
        .collect()
}

fn count_score(vector: Vec<(Shape, Shape)>) -> i64 {
    vector
        .into_iter()
        .map(|(left, right)| match (&left, &right) {
            _ if left.defeated_by() == right => Win.score() + right.score(),
            _ if left.defeats() == right => Lose.score() + right.score(),
            _ => Draw.score() + right.score(),
        })
        .sum()
}

pub(crate) fn problem2_1() -> i64 {
    let vector = parse()
        .into_iter()
        .map(|(left, right_char)| {
            let right = match right_char {
                'X' => Some(Rock),
                'Y' => Some(Paper),
                'Z' => Some(Scissors),
                _ => None
            }.expect("value to be X, Y or Z");
            (left, right)
        })
        .collect();

    count_score(vector)
}

pub(crate) fn problem2_2() -> i64 {
    let vector = parse()
        .into_iter()
        .map(|(left, right_char)| {
            let right = match right_char {
                'X' => Some(Lose),
                'Y' => Some(Draw),
                'Z' => Some(Win),
                _ => None
            }.expect("value to be X, Y or Z");
            (left, right)
        })
        .map(|(left, result)| match (&left, result) {
            (_, Win) => (left, left.defeated_by()),
            (_, Lose) => (left, left.defeats()),
            _ => (left.clone(), left)
        })
        .collect();

    count_score(vector)
}

