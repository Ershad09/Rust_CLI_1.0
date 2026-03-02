
use std::env;

fn main(){
    let args: Vec<String> = env::args().skip(1).collect();

    let upper_text = args.join(" ").to_uppercase();

    println!("{}", upper_text);
}