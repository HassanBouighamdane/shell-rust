#[allow(unused_imports)]
use std::io::{self, Write};
use std::vec;

fn main() {
    let shell_commands=vec!["echo", "exit","type"];
    loop {
        // Uncomment this block to pass the first stage
     print!("$ ");
     io::stdout().flush().unwrap();


    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();

/* version 1
    if input.starts_with("exit 0"){
        break;
    }else if input.starts_with("echo"){
        let value=input.split_off(5);
        print!("{value}");
    }else {
        println!("{}: command not found",input.trim());
    }
    */
    // Optimized version with match
    match input.trim(){
        "exit 0"=> break,
        input if input.starts_with("echo ")=>{
            println!("{}",&input[5..]);
        }
        input if input.starts_with("type ")=>{
           if shell_commands.contains(&&input[5..]){
            println!("{} is a shell builtin",&input[5..]);
           }else {
            println!("{}: not found",&input[5..]);
        }
        }
        &_=>{
            println!("{}: command not found",input.trim());
        }
    }

    }
}
