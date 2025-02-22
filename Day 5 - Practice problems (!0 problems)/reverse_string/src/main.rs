//2️⃣ Reverse a String

// Given a string, reverse it without using .reverse().
// Handle UTF-8 characters correctly.

use std::io;

fn main() {

    let mut user_input = String::new();
    
    println!("Please enter some text:");
    io::stdin()
        .read_line(&mut user_input)
        .expect("failed to read line");

    let user_input = user_input.trim();

    let reversed_input:String = user_input.chars().rev().collect();

    println!("{}",reversed_input)
}
