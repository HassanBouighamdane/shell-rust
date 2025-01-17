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
    }else if input.starts_with("echo"){
        let value=input.split_off(5);
        print!("{value}");
    }else {
        println!("{}: command not found",input.trim());
    }

    }
   
    
}
