use std::{env, process};




fn main(){
    let args:Vec<String> = env::args().skip(1).collect();

    if args.is_empty(){
        eprintln!("Usage: cargo run -- <text>");
        process::exit(1);
    }

    let text = args.join(" ");

    let add_exclamation = format!("{}!!!", text);

    println!("{}", add_exclamation);

}

