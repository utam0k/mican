extern crate libc;
extern crate rtsh;

use std::io::{stdin, stdout, Write};
use std::fs;
use std::env;
use rtsh::parser;

fn rtsh_cd(args: &parser::CommandData) -> Result<(), String> {
    if args.options.len() < 1 {
        env::set_current_dir(&env::home_dir().unwrap()).unwrap();
        return Ok(());
    }

    let mut current_path_buf = std::env::current_dir().unwrap();
    current_path_buf.push(&args.options[0]);
    if env::set_current_dir(current_path_buf.as_path()).is_err() {
        return Err(format!("{} not found", args.options[0]));
    };
    return Ok(());
}

fn rtsh_ls(_args: &parser::CommandData) -> Result<(), String> {
    for entry in fs::read_dir(env::current_dir().unwrap()).unwrap() {
        println!("{:?}", entry.unwrap().file_name());
    }
    return Ok(());
}

fn rtsh_pwd() -> Result<(), String> {
    println!("{:?}", env::current_dir().unwrap());
    return Ok(());
}

fn main() {
    println!("Welcome to rust shell.");

    loop {
        print!("> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).ok().expect("Failed to read.");

        input.pop().unwrap();
        let commands = parser::Parser {
            pos: 0,
            input: input,
        }.parse();

        let c0 = &commands[0];
        let _ = match c0 {
            &parser::Token::Command(ref c) => match c.program.as_str() {
                "cd" => rtsh_cd(&c),
                "ls" => rtsh_ls(&c),
                "pwd" => rtsh_pwd(),
                _ => Err(format!("not found {} command.", c.program)),
            },
            _ => Err(format!("not found command.")),
        }.map_err(|err| eprintln!("{}", err));
    }
}
