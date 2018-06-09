extern crate mican;

use std::error::Error;
use std::fs;
use std::io::prelude::*;
use std::path::Path;

use mican::commands;
use mican::parser;
use mican::process::Process;
use mican::readline::reader::Reader;
use mican::readline::context::Context;
use mican::readline::completer::Bin as BinCompleter;

fn display_logo() {
    let path = Path::new("logo.txt");

    let mut file = match fs::File::open(&path) {
        Err(why) => panic!("couldn't open: {}", Error::description(&why)),
        Ok(file) => file,
    };

    let mut s = String::new();

    if let Err(e) = file.read_to_string(&mut s) {
        panic!("couldn't read: {}", Error::description(&e));
    } else {
        for c in s.chars() {
            match c {
                '&' => print!("\x1B[38;5;212170m&\x1B[0m"),
                '8' => print!("\x1B[38;5;70m8\x1B[0m"),
                '#' => print!("\x1B[38;5;9346m#\x1B[0m"),
                s => print!("{}", s),
            }
        }
    };
}

fn waitpids(children: Vec<Process>) {
    for c in children {
        if let Err(e) = c.wait() {
            println!("Error!: {:?}", e);
        };
    }
}

fn main() {
    display_logo();
    println!("Welcome to Mican Unix Shell.");
    let mut reader = Reader::new(Context::new(Box::new(BinCompleter::new())));

    loop {
        if let Some(input) = reader.read_line() {
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
}
