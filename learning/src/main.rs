mod math;
mod mod_vars_in_rust;
mod vars_in_rust;
//use math;
// mod math and use math should not be used together
//use std::{io::stdin, u64};

use crate::math::{
    add,
    divide,
    //exponent,
    square,
    subtract,
};
fn main() {
    let x = 3;
    let y = 10;

    let sum = math::add(x, y);
    let y_plus_2 = math::add(y, 2);
    let result = math::multiply(sum, y_plus_2);
    let five = add(5, 4);
    let limga = math::add(10, 10);
    println!("x + y = {}", sum);
    println!("y + 2 = {}", y_plus_2);
    println!("(x + y) * (y + 2) = {}", result);
    println!("five = {five}\n adn limga = {limga}");
    println!("10 = {}", subtract(10, 12));
    println!("10 = {}", add(54, 1));

    //  println!("Hello, world!");

    let mut dividant_string: String = String::new();
    let mut divider_string: String = String::new();
    println!("ENTER THE NUMBER YOU WANT BE DIVIDED");
    std::io::stdin()
        .read_line(&mut dividant_string)
        .expect("EROR READING DIVIANT");

    println!("ENTER THE NUMBER U WANT TO DIVIDE WITH");
    std::io::stdin()
        .read_line(&mut divider_string)
        .expect("COULD NOT READ DIIVVER");

    let dividant: i32 = dividant_string
        .trim()
        .parse()
        .expect("COULD NOT CONERT DIVIDANTSTRING INTO INT");
    let divider: i32 = divider_string
        .trim()
        .parse()
        .expect("COULD NOT CONVERT DIVIDERSTRING TO INT");

    let answer: i32 = divide(dividant, divider);

    println!("THE ANSWER OF {}/{} is {}", dividant, divider, answer);

    let nice: i32 = square(10);
    println!("SQUARE of 10 IS {}, (or {})", nice, square(10));

    let no1: i32 = 20;
    let no2: i32 = 10;
    //    let answerto_nno: i32 = exponent(no1, no2);
    //let answerto_nno2: i32 = exponent(no2, no1);
    let answerto_nno2 = 10;
    let answerto_nno = 20;
    println!(
        "{} powered by {} is {}, and {} powered by {} is {}",
        no1, no2, answerto_nno, no2, no1, answerto_nno2
    );
}
