// simple CLI 
use clap::Parser;

//instead of thinking them as text, we could represent the CLI args as a custom datatype using Struct
#[derive(Parser)]
struct Cli{
    pattern:String,
    path:std::path::PathBuf,
}
fn main() {
    let pattern = std::env::args().nth(1).expect("no pattern given");
    let path = std::env::args().nth(2).expect("no path given");

    let args = Cli {
        pattern: pattern.clone(), 
        path: std::path::PathBuf::from(path),
    };

    println!("pattern: {:?}, path: {:?}",& args.pattern,args.path)


}
