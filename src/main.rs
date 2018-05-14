extern crate mican;

use mican::commands;
use mican::parser;
use mican::process::Process;
use mican::readline::reader::Reader;

use std::error::Error;
use std::fs;
use std::io::prelude::*;
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
        Ok(_) => {
            for c in s.chars() {
                match c {
                    '&' => print!("\x1B[38;5;{}m&\x1B[0m", 212170),
                    '8' => print!("\x1B[38;5;{}m8\x1B[0m", 70),
                    '#' => print!("\x1B[38;5;{}m#\x1B[0m", 9346),
                    s => print!("{}", s),
                }
            }
        }
    };
}

fn waitpids(children: Vec<Process>) {
    for c in children {
        c.wait()
    }
}

fn main() {
    display_logo();
    println!("Welcome to Mican Unix Shell.");
    let mut reader = Reader::new("> ".into());

    loop {
        let input = reader.read_line();

        let commands = parser::Parser::new(input).parse();

        let mut children: Vec<Process> = Vec::new();
        for c in commands {
            let p = match c.program.as_str() {
                "cd" => Process::new(commands::cd::run),
                "ls" => Process::new(commands::ls::run),
                "pwd" => Process::new(commands::pwd::run),
                "clear" => Process::new(commands::clear::run),
                "bye" => Process::new(commands::bye::run),
                "tanakh" => Process::new(commands::tanakh::run),
                "syar" => Process::new(commands::syar::run),
                _ => Process::new(commands::other::run),
            };
            if p.in_child() {
                let _ = p.run(c).map_err(|err| eprintln!("{}", err));
            } else {
                children.push(p)
            }
        }
        waitpids(children);
    }
}
