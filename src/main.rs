#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::{
    self,Path,PathBuf
};

use std::vec;
use std::env;
use std::process::Command;



fn main() {
    
    loop {
     print!("$ ");
     io::stdout().flush().unwrap();


    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();

    input =input.trim().to_string();

    let args=parse_input(&input);
    
    if args.is_empty(){
        continue;
    } 

    let shell_commands=vec!["echo", "exit","type","pwd","cd"];
    let path=env::var("PATH").unwrap();

    match args[0].as_str(){

        // The exit command
        "exit"=> {
            
            if args.len() == 2 && args[1] == "0" {
                break;
            } else {
                println!("exit requires a valid argument");
            }
        }

        //The echo command
         "echo"=>{
            println!("{}",args[1..].join(" "));
        },

        //The type command
        "type"=>{
           if args.len()==2{
            let cmd=args[1].as_str();
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
        // The pwd command
        "pwd"=>{
            if args.len()>1{
                println!("The pwd command has 0 argument but {} found", args.len()-1);
            }
            else{
                let dir=env::current_dir().unwrap();
                println!("{}",dir.display());
                //the following can also be used
                // println!("{}",dir.to_str().unwrap());
                
            }
        }

        //the cd command
        "cd"=>{
                let path=if args.len()==2{
                    args[1].to_string()
                }else{
                    String::new()
                };
                
                //this part is used to get the home directory based on the os (linux or windows)
                 let home_dir = if cfg!(windows) {
                    env::var("USERPROFILE").unwrap()
                } else {
                    env::var("HOME").unwrap()
                };

                let path = if path.is_empty() || (path == "~") {
                    home_dir
                } else if path.starts_with(path::MAIN_SEPARATOR) {
                    path
                } else {
                    format!("{}/{}", env::current_dir().unwrap().display(), path)
                };


                let path = Path::new(&path);
                if path.exists() && path.is_dir() {
                    env::set_current_dir(path).unwrap();
                } else {
                    eprintln!("cd: {}: No such file or directory", path.display());
                }
        } 
    
        
        _=>{
            let exec=args[0].as_str();
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


// Function to parse input with support for single and double quotes
fn parse_input(input: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut in_single_quotes = false;
    let mut in_double_quotes = false;
    let mut escape_next = false;

    for c in input.chars() {
        if escape_next {
            // Handle escaped characters in double quotes
            match c {
                '$' | '`' | '"' | '\\' => current.push(c),
                '\n' => {} // Ignore escaped newline
                _ => {
                    current.push('\\');
                    current.push(c);
                }
            }
            escape_next = false;
        } else {
            match c {
                '\\' if !in_double_quotes => {
                    continue;
                }
                '\\' if in_double_quotes => escape_next = true,
                
                '"' if !in_single_quotes => {
                    in_double_quotes = !in_double_quotes;
                    if !in_double_quotes {
                        // End of double quotes: expand variables and backticks
                        current = expand_variables_and_backticks(&current);
                    }
                }
                '\'' if !in_double_quotes => {
                    in_single_quotes = !in_single_quotes;
                }
                ' ' | '\t' if !in_single_quotes && !in_double_quotes => {
                    if !current.is_empty() {
                        args.push(current.clone());
                        current.clear();
                    }
                }
                _ => current.push(c),
            }
        }
    }

    if !current.is_empty() {
        if in_double_quotes {
            current = expand_variables_and_backticks(&current);
        }
        args.push(current);
    }

    args
}

// Function to expand variables (e.g., $VAR -> value) and handle backticks
fn expand_variables_and_backticks(input: &str) -> String {
    let mut result = String::new();
    let mut chars = input.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '$' => {
                let mut var_name = String::new();
                while let Some(&next) = chars.peek() {
                    if next.is_alphanumeric() || next == '_' {
                        var_name.push(next);
                        chars.next();
                    } else {
                        break;
                    }
                }
                if let Ok(value) = env::var(&var_name) {
                    result.push_str(&value);
                }
            }
            '`' => {
                let mut command = String::new();
                while let Some(&next) = chars.peek() {
                    if next == '`' {
                        chars.next(); // Consume closing backtick
                        break;
                    } else {
                        command.push(next);
                        chars.next();
                    }
                }
                if let Ok(output) = Command::new("sh")
                    .arg("-c")
                    .arg(command)
                    .output()
                {
                    if let Ok(output_str) = String::from_utf8(output.stdout) {
                        result.push_str(output_str.trim());
                    }
                }
            }
            _ => result.push(c),
        }
    }
    result
}