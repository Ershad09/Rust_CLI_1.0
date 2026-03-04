use std::env;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        println!("Usage: <text>");
        return;
    }

    let text = args.join(" ");

    println!("Input: {}", text);
    
    println!("Byte Length: {}", text.len());
    println!("Character Count: {}", text.chars().count());
    println!("Visual Length: {}", text.graphemes(true).count());
}
