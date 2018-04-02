mod parser;
mod commands;

use std::io::{stdin, stdout, Write};
use std::error::Error;
use std::io::prelude::*;
use std::fs;
use std::path::Path;

fn display_logo() {
    let path = Path::new("logo.txt");

    let mut file = match fs::File::open(&path) {
        Err(why) => panic!("couldn't open: {}", Error::description(&why)),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read: {}", Error::description(&why)),
        Ok(_) => for c in s.chars() {
            match c {
                '&' => print!("\x1B[38;5;{}m&\x1B[0m", 212170),
                '8' => print!("\x1B[38;5;{}m8\x1B[0m", 70),
                '#' => print!("\x1B[38;5;{}m8\x1B[0m", 9346),
                s => print!("{}", s),
            }
        },
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
        let commands = parser::parser::Parser::new(input).parse();

        for c in commands {
            match c {
                parser::parser::Token::Command(c) => {
                    let _ = match c.program.as_str() {
                        "cd" => commands::cd::run(&c),
                        "ls" => commands::ls::run(&c),
                        "pwd" => commands::pwd::run(),
                        "clear" => commands::clear::run(),
                        _ => commands::other::run(&c),
                    }.map_err(|err| eprintln!("{}", err));
                }
                parser::parser::Token::Pipe => println!("pipe"),
            };
        }
    }
}
