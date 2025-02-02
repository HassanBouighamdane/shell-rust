use std::path::{Path, PathBuf};
use std::process::exit;
use std::process::Command;
use std::{env, fs};
#[allow(unused_imports)]
use std::io::{self, Write};
fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    print!("$ ");
    io::stdout().flush().unwrap();
    while stdin.read_line(&mut input).is_ok() {
        let input_string = input.strip_suffix('\n').unwrap();
        if let Some(command) = parse_input(input_string) {
            match command {
                ShellCommand::EXIT(val) => exit(val),
                ShellCommand::ECHO(argument) => {
                    println!("{}", argument);
                }
                ShellCommand::TYPE(argument) => match type_of_command(&argument) {
                    CommandType::Builtin => {
                        println!("{} is a shell builtin", argument);
                    }
                    CommandType::Program(path) => {
                        println!("{} is {}", argument, path.to_str().unwrap());
                    }
                    CommandType::Nonexistent => {
                        println!("{}: not found", argument);
                    }
                },
                ShellCommand::PWD() => {
                    println!("{}", std::env::current_dir().unwrap().to_str().unwrap())
                }
                ShellCommand::CD(argument) => {
                    let home = std::env::var("HOME").unwrap();
                    if std::env::set_current_dir(Path::new(&argument.replace("~", &home))).is_err()
                    {
                        println!("cd: {}: No such file or directory", argument);
                    }
                }
                ShellCommand::Program((command, arguments)) => {
                    let command_type = type_of_command(command);
                    let args = parse_arguments(&arguments);
                    match command_type {
                        CommandType::Nonexistent => {
                            println!("{}: command not found", input_string);
                        }
                        CommandType::Program(path) => {
                            let output = Command::new(path)
                                .args(args)
                                .output()
                                .expect("fail to run program");
                            print!("{}", String::from_utf8_lossy(&output.stdout))
                        }
                        CommandType::Builtin => {}
                    };
                }
            }
        } else {
            println!("{}: command not found", input_string);
        }
        input.clear();
        print!("$ ");
        io::stdout().flush().unwrap();
    }
}
#[derive(Debug, Clone)]
pub enum ShellCommand<'a> {
    EXIT(i32),
    ECHO(String),
    CD(String),
    TYPE(String),
    PWD(),
    Program((&'a str, String)),
}
fn parse_arguments(arguments: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current_arg = String::new();
    let mut in_single_quotes = false;
    let mut in_double_quotes = false;
    let mut escaped = false;
    for c in arguments.chars() {
        match c {
            '"' if !in_single_quotes && !escaped => in_double_quotes = !in_double_quotes,
            '\'' if !in_double_quotes && !escaped => in_single_quotes = !in_single_quotes,
            ' ' if !in_single_quotes && !in_double_quotes && !escaped => {
                if !current_arg.is_empty() {
                    args.push(current_arg.clone());
                    current_arg.clear();
                }
            }
            '\\' if !in_single_quotes && !in_double_quotes && !escaped => escaped = true,
            _ => {
                escaped = false;
                current_arg.push(c)
            }
        }
    }
    if !current_arg.is_empty() {
        args.push(current_arg);
    }
    args
}
fn parse_input(input: &str) -> Option<ShellCommand> {
    let (command, arguments) = match input.find(' ') {
        Some(_index) => input.split_once(' ')?,
        None => (input, ""),
    };
    let parsed_args = parse_arguments(arguments).join(" ");
    match command {
        "exit" => Some(ShellCommand::EXIT(parsed_args.parse::<i32>().unwrap())),
        "echo" => Some(ShellCommand::ECHO(parsed_args)),
        "type" => Some(ShellCommand::TYPE(parsed_args)),
        "pwd" => Some(ShellCommand::PWD()),
        "cd" => Some(ShellCommand::CD(parsed_args)),
        _default => Some(ShellCommand::Program((command, arguments.to_string()))),
    }
}
#[derive(Debug, Clone)]
pub enum CommandType {
    Builtin,
    Nonexistent,
    Program(PathBuf),
}
fn type_of_command(command: &str) -> CommandType {
    match command {
        "echo" => CommandType::Builtin,
        "exit" => CommandType::Builtin,
        "type" => CommandType::Builtin,
        "pwd" => CommandType::Builtin,
        "cd" => CommandType::Builtin,
        _default => {
            if let Ok(path) = env::var("PATH") {
                let paths: Vec<&str> = path.split(':').collect();
                for path in paths.iter() {
                    let folder = match fs::read_dir(path) {
                        Ok(fold) => fold,
                        Err(_err) => continue,
                    };
                    for item in folder.into_iter() {
                        if item.as_ref().unwrap().file_name() == command {
                            return CommandType::Program(item.unwrap().path());
                        }
                    }
                }
                let full_path = Path::new(command);
                if full_path.exists() {
                    return CommandType::Program(full_path.to_path_buf());
                }
                CommandType::Nonexistent
            } else {
                CommandType::Nonexistent
            }
        }
    }
}