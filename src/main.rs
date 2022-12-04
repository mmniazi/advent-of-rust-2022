#![feature(binary_heap_into_iter_sorted)]
#![feature(iter_array_chunks)]
#![feature(drain_filter)]

use problem1::*;
use problem2::*;
use problem3::*;
use problem4::*;

mod problem1;
mod problem2;
mod problem3;
mod problem4;

fn main() {
    println!("problem1_1: {}", problem1_1());
    println!("problem1_2: {}", problem1_2());
    println!("problem2_1: {}", problem2_1());
    println!("problem2_2: {}", problem2_2());
    println!("problem3_1: {}", problem3_1());
    println!("problem3_2: {}", problem3_2());
    println!("problem4_1: {}", problem4_1());
    println!("problem4_2: {}", problem4_2());
}
