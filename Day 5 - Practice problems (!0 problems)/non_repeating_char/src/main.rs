// Find the First Non-Repeating Character

// Given a string, return the first character that does not repeat.
use std::io;
use std::collections::HashMap;
fn main() {
    let mut user_input = String::new();
    
    println!("Please enter some text:");
    io::stdin()
        .read_line(&mut user_input)
        .expect("failed to read line");

    let user_input: &str = user_input.trim();

    let mut map = HashMap::new();
    for c in user_input.chars(){
        let count = map.entry(c).or_insert(0);
        *count +=1;
    }
    for c in user_input.chars(){
        if let Some(count) = map.get(&c){
        if *count == 1{
            println!("{} is the key having first unique occurence",c);
            break;
        }
    }
    }
}
