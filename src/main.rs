#![allow(non_snake_case)]
use std::{i8, i16, i32, i64, i128, u8, u16, u32, u64, isize, usize, f32, f64};

use std::io::stdin;

fn main() {
    println!("Hello, Rust !!!");

    let num: i8 = 10; //i32 by default
    println!("this number is {}", num);
    let name: &str = "Vedansh";
    let sur_name: &str = "Shrivastava";
    println!("My name is {} {}", name, sur_name);
}
