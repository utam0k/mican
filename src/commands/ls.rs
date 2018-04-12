use token::CommandData;

use std::env;
use std::fs;
use std::io::prelude::*;
use std::os::unix::ffi::OsStrExt;

pub fn run(cmd: CommandData) -> Result<(), String> {
    let mut out = cmd.out.unwrap();
    for entry in fs::read_dir(env::current_dir().unwrap()).unwrap() {
        match out.write_all(entry.unwrap().file_name().as_bytes()) {
            Ok(_) => {
                out.write_all("\n".as_bytes()).unwrap();
            }
            Err(_) => return Err("Error: pwd".to_string()),
        }
    }
    return Ok(());
}
