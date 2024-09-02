/*
 * @Description: 
 * @Author: didadida262
 * @Date: 2024-09-03 00:35:15
 * @LastEditors: didadida262
 * @LastEditTime: 2024-09-03 00:36:51
 */
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}