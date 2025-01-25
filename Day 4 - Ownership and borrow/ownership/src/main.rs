fn main() {
}

fn ex1(){
    let s1 = String::from("Hello world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}

fn take_ownership(s: String)->String {
    println!("{}", s);
    s
}

fn ex3(){
     let s = give_ownership();
    println!("{}", s);
}

fn give_ownership() -> String {
    let s = String::from("Hello world");
    s
}

fn ex4(){
    let s = String::from("Hello World");

    print_str(&s);

    println!("{}", s);
}

fn print_str(s: &String)  {
    println!("{}",s)
}

fn ex5(){
     let x = (1, 2, (), "hello");
    let y = x;
    println!("{:?}, {:?}", x, y);
}


fn ex6(){
    let s = String::from("Hello ");
    
    let mut s1 = s;

    s1.push_str("World!");

    println!("Success!");
}

fn ex7(){
      let x = Box::new(5);
    
     let mut y = Box::new(3);      // update this line, don't change other lines!
    
    *y = 4;
    
    assert_eq!(*x, 5);

    println!("Success!");
}

fn ex8(){
    let t = (String::from("hello"), String::from("world"));
 
    let _s = t.0;
 
    // modify this line only, don't use `_s`
    println!("{:?}", t.1);
}

fn ex9(){
     let t = (String::from("hello"), String::from("world"));

    // Fill the blanks
    let (s1,s2) = t.clone();

    println!("{:?}, {:?}, {:?}", s1, s2, t); 
}