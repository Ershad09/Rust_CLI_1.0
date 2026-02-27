


use std::env;


fn main(){
    let args: Vec<String> = env::args().collect();

    let name = match args.get(1) {
        Some(n) => n,
        None => {
            eprintln!("Please Provide a name");
            eprintln!("Usage: greet<name>");
            return;
        }
    };
        println!("Hello {}", name);
}




