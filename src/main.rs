extern crate libc;

use std::io::{Write, stdout, stdin};
use std::fs;
use std::env;
use std::ffi::CString;

enum Command {}

fn rtsh_cd(args: Vec<&str>) -> Result<(), String> {
    if args.len() < 2 {
        // env::set_current_dir(&env::home_dir().unwrap()).unwrap();
        // use std::path::Path;
        // env::set_current_dir(Path::new("/home/utam0k/")).unwrap();
        unsafe {
            if libc::chdir(CString::new("/home/utam0k").unwrap().as_ptr()) == -1 {
                return Err(format!("{} not found", args[1]));
            };
        }
        return Ok(());
    }
    // let mut current_path_buf = std::env::current_exe().unwrap();
    // current_path_buf.push(args[1]);
    // println!("{:?}", current_path_buf);
    // if env::set_current_dir(current_path_buf.as_path()).is_err(){
    //     return Err(format!("{} not found", args[1]));
    // };
    // return Ok(());
    unsafe {
        if libc::chdir(CString::new(args[1]).unwrap().as_ptr()) == -1 {
            return Err(format!("{} not found", args[1]));
        };
    }
    return Ok(())
}

fn rtsh_ls(args: Vec<&str>) -> Result<(), String> {
    for entry in fs::read_dir(env::current_exe().unwrap()).unwrap() {
        println!("{:?}", entry.unwrap().file_name());
    }
    return Ok(());
}

fn rtsh_pwd() -> Result<(), String> {
    println!("{:?}", env::current_exe().unwrap());
    return Ok(());
}

fn main() {
    println!("Welcome to rust shell.");

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
            "cd" => rtsh_cd(args),
            "ls" => rtsh_ls(args),
            "pwd" => rtsh_pwd(),
            _    => Err(format!("not found {} command.", cmd)),
        }.map_err(|err| eprintln!("{}", err));
    }
}
