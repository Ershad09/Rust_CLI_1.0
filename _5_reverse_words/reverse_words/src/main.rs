use std::env;


fn main(){

        //  collecting command line arguments  into a vector of string
        let word:Vec<String> = env::args().skip(1).collect();

        if word.is_empty(){
            println!("Usage: cargo run -- <text>"); 
        }

        // converting vector to string
        let  words = word.join(" ");

        //  convert the string into charcters , reverse theme(.rev()), and then collect into a new String
        let reverse_words: String = words.chars().rev().collect(); 

        println!("input: {}" , words);
        println!("reverse: {}", reverse_words);                

}

