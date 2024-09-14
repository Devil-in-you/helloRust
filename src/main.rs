/**
 * Author: Vedansh Shrivastava
 * date: 10/10/21
 * catch me at: https://www.linkedin.com/in/vedansh-shrivastava-b5ab221b3/
 */


#[allow(non_snake_case)]// if you want to ignore the snake case warning
use std::{i8, i16, i32, i64, i128, u8, u16, u32, u64, isize, usize, f32, f64};

use std::io::stdin;

fn main() {
    println!("Hello, Rust !!!");

    let num: i8 = 10; //i32 by default
    println!("this number is {}", num);
    let name: &str = "Vedansh";
    let sur_name: &str = "Shrivastava";
    println!("My name is {} {}", name, sur_name);

    let  mut num = 100;
    println!("the number is {}", num);

    //Boolean
    let is_it_true: bool = true; //false

    //Character
    let one_char: char = 'a'; //single quotes not double quotes

    //place variable in placeholders
    println!("my name is {} {}, and my fav char is {}", name, sur_name, one_char);

    //Define multiple variables on multiple lines
    let(country, city) = ("India", "Bhopal");
    println!("I am from {} and I live in {}", country, city);

    //Operators for math calculations
    println!("5 + 4 = {}", 5 + 4);
    println!("5 - 4 = {}", 5 - 4);
    println!("5 * 4 = {}", 5 * 4);
    println!("5 / 4 = {}", 5 / 4);
    println!("5 % 4 = {}", 5 % 4);

}
