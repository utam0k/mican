extern crate rtsh;
extern crate libc;

use std::io::{Write, stdout, stdin};
use std::fs;
use std::env;
use rtsh::parser;

enum Command {}

fn rtsh_cd(args: Vec<&str>) -> Result<(), String> {
    if args.len() < 2 {
        env::set_current_dir(&env::home_dir().unwrap()).unwrap();
        return Ok(());
    }

    let mut current_path_buf = std::env::current_dir().unwrap();
    current_path_buf.push(args[1]);
    if env::set_current_dir(current_path_buf.as_path()).is_err(){
        return Err(format!("{} not found", args[1]));
    };
    return Ok(())
}

fn rtsh_ls(args: Vec<&str>) -> Result<(), String> {
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
    println!("{:?}", parser::Parser{pos: 0, input: "ls -al".to_string()}.parse());

    loop {
        print!("> ");
        stdout().flush().unwrap();

        let mut cmd = String::new();
        stdin().read_line(&mut cmd)
            .ok()
            .expect("Failed to read.");

        cmd.pop().unwrap();
        let args: Vec<&str> = cmd.split(" ").collect();

        let _ = match args[0] {
            "cd"  => rtsh_cd(args),
            "ls"  => rtsh_ls(args),
            "pwd" => rtsh_pwd(),
            _     => Err(format!("not found {} command.", cmd)),
        }.map_err(|err| eprintln!("{}", err));
    }
}
