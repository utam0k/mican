use token::CommandData;

use std::env;
use std::io::prelude::*;

pub fn run(cmd: CommandData) -> Result<(), String> {
    let result = format!("{}\n", env::current_dir().unwrap().display());
    let mut out = cmd.out.unwrap();
    match out.write_all(result.as_bytes()) {
        Ok(_) => Ok(()),
        Err(_) => Err("Error: pwd".to_string()),
    }
}
