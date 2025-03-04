

//RUST OPTIONS


//Write a function unwrap_value that takes an Option<i32> and returns the value inside it. If it's None, return 0.

fn unwrap_value (opt: Option<i32>)->i32{
    opt.unwrap_or(0)                 // unwrap is used on Option and Result enum. Normal unwrap panicks but we use unwrap_or for graceful error handling
}

fn unwrap_value_expect(opt: Option<i32>)->i32{
    opt.expect("None occured")
}

//----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------


fn main() {

    // basic option wrap
    println!("{}",unwrap_value(Some(42)));
    println!("{}",unwrap_value(None));
    println!("{}",unwrap_value_expect(None));


}
