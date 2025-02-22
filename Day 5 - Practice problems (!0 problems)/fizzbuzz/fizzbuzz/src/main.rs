// FIZZBUZZ
//
//
// Write a program that prints numbers from 1 to 100:
// If a number is divisible by 3, print "Fizz".
// If divisible by 5, print "Buzz".
// If divisible by both, print "FizzBuzz".


fn main() {
    for i in 0..101{
        if i % 5==0 && i % 3 ==0{
            println!("Fizzbuzz");
        }
        else if i%3 ==0 {
            println!("Fizz");
        }
        else if i % 5 ==0 {
            println!("Buzz");
        }
        else {continue};
    }
}
