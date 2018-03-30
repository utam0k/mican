extern crate mican;

use std::io::{stdin, stdout, Write};
use std::fs;
use std::env;
use std::error::Error;
use std::io::prelude::*;
use std::path::Path;
use mican::parser;

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

fn rtsh_clear() -> Result<(), String> {
    stdout().write_all("\x1b[2J\x1b[1;1H".as_bytes()).unwrap();
    return Ok(());
}

fn display_logo() {
    let path = Path::new("logo.txt");

    let mut file = match fs::File::open(&path) {
        Err(why) => panic!("couldn't open: {}", Error::description(&why)),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read: {}", Error::description(&why)),
        Ok(_) => {
            for c in s.chars() {
                match c {
                    '&' => print!("\x1B[38;5;{}m&\x1B[0m", 166),
                    '8' => print!("\x1B[38;5;{}m&\x1B[0m", 64),
                    s => print!("{}", s),
                }
            }
        }
    };
}

fn main() {
    display_logo();
    println!("Welcome to Mican Unix Shell.");

    loop {
        print!("> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).ok().expect("Failed to read.");

        input.pop().unwrap();
        let commands = parser::Parser::new(input).parse();

        for c in commands {
            match c {
                parser::Token::Command(c) => {
                    let _ = match c.program.as_str() {
                        "cd" => rtsh_cd(&c),
                        "ls" => rtsh_ls(&c),
                        "pwd" => rtsh_pwd(),
                        "clear" => rtsh_clear(),
                        _ => Err(format!("not found {} command.", c.program)),
                    }.map_err(|err| eprintln!("{}", err));
                }
                parser::Token::Pipe => println!("pipe"),
            };
        }
    }
}
