use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;
fn main() {
    println!("Guess game");
    

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is {}",secret_number);

    loop {
        let mut guess = String::new();
        println!("Please input a guess");
    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");

    let guess:u32 = match guess.trim().parse(){
        Ok(num) => num,
        Err(_) => continue,  
    };
    println!("you guessed: {}",guess);
    match guess.cmp(&secret_number){
        Ordering::Less => println!("{}","Too Small".red()),
        Ordering::Greater => println!("{}","Too Big".red()),
        Ordering::Equal => 
        {
            println!("{}","You Win".green());
            break;
        }
    }
    }
    
}
