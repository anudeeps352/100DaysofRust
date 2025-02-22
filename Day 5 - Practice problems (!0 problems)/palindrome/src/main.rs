// 4️⃣ Check for Palindrome

// Implement a function that checks if a given string is a palindrome.
// Example: "racecar" → ✅, "hello" → ❌

use std::io;

fn main() {
    println!("Please enter to check whether something is palindrome");

    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    let trimmed_input = user_input.trim();

    let reversed_string:String = trimmed_input.chars().rev().collect();

    if user_input.trim() == reversed_string{
        println!("It is a palindrome");
    }
    else{
        println!("Its not a palindrome")
    }

}
