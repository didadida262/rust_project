/*
 * @Description: 
 * @Author: didadida262
 * @Date: 2024-09-03 00:35:15
 * @LastEditors: didadida262
 * @LastEditTime: 2024-09-03 10:43:03
 */
use std::io;

fn main() {
    println!("Please enter a number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let number: i32 = input.trim().parse().expect("Please enter a valid number");

    if number < 0 {
        println!("The number is negative.");
    } else if number == 0 {
        println!("The number is zero.");
    } else {
        println!("The number is positive.");
    }
}