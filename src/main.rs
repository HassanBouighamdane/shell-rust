#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {

    loop {
        // Uncomment this block to pass the first stage
     print!("$ ");
     io::stdout().flush().unwrap();


    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    
    if input.starts_with("exit 0"){
        break;
    };
    println!("{}: command not found",input.trim());

    
    }
   
    
}
