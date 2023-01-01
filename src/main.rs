#![feature(binary_heap_into_iter_sorted)]
#![feature(iter_array_chunks)]
#![feature(drain_filter)]
#![feature(let_chains)]

use problem1::*;
use problem2::*;
use problem3::*;
use problem4::*;
use problem5::*;
use problem6::*;
use problem7::*;
use problem8::*;

mod problem1;
mod problem2;
mod problem3;
mod problem4;
mod problem5;
mod problem6;
mod problem7;
mod problem8;

fn main() {
    println!("problem1_1: {}", problem1_1());
    println!("problem1_2: {}", problem1_2());
    println!("problem2_1: {}", problem2_1());
    println!("problem2_2: {}", problem2_2());
    println!("problem3_1: {}", problem3_1());
    println!("problem3_2: {}", problem3_2());
    println!("problem4_1: {}", problem4_1());
    println!("problem4_2: {}", problem4_2());
    println!("problem5_1: {}", problem5(false));
    println!("problem5_2: {}", problem5(true));
    println!("problem6_1: {}", problem6(false));
    println!("problem6_2: {}", problem6(true));
    println!("problem7_1: {}", problem7_1());
    println!("problem7_2: {}", problem7_2());
    println!("problem8_1: {}", problem8_1());
    println!("problem8_2: {}", problem8_2());
}
