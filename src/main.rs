#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::PathBuf;
use std::vec;
use std::env;
use std::process::Command;

fn find_exec(name:&str)-> Option<PathBuf>{
    if let Ok(paths) = env::var("PATH") {
        for path in env::split_paths(&paths) {
            let exe_path = path.join(name);
            if exe_path.is_file() {
                return Some(exe_path);
            }
        }
    }
    None
}

fn main() {
    
    loop {
     print!("$ ");
     io::stdout().flush().unwrap();


    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();

    let args=input.split_whitespace().collect::<Vec<&str>>();
    
    if args.is_empty(){
        continue;
    } 

    let shell_commands=vec!["echo", "exit","type"];
    let path=env::var("PATH").unwrap();

    match args[0]{

        // The exit command
        "exit"=> {
            if args.len()==2{
                match args[1]{
                    "0"=> break,
                    _=>{
                        println!("{} is not a valid exit argument",args[1]);
                    }
                }
            }else{
                println!("exit needs a valid argument");
            }
        }

        //The echo command
         "echo"=>{
            print!("{}",&input[5..]);
        },

        //The type command
        "type"=>{
           if args.len()==2{
            let cmd=args[1];
                if shell_commands.contains(&cmd){
                    println!("{} is a shell builtin",cmd);
                }
                else{
                    let splited_path=&mut path.split(":");
                    if let Some(path) =
                    splited_path.find(|path| std::fs::metadata(format!("{}/{}", path, cmd)).is_ok())
                    {
                    println!("{cmd} is {path}/{cmd}");
                    } else {
                    println!("{cmd} not found");
                    }
                }
           }

           else{
            println!("the type arguments are not valid");
           }
        },
        
        _=>{
            let exec=args[0];
            if find_exec(exec)!=None{
                Command::new(exec)
                .args(&args[1..])
                .status()
                .expect("failed to execute the program");
            }
            else{
                println!("{}: command not found",args[0]);
            }
        }
    }
    }
}
