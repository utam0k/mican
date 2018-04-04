use std::env;
use std::fs;
use std::io::prelude::*;

pub fn run(mut out: &fs::File) -> Result<(), String> {
    let result = format!("{:?}\n", env::current_dir().unwrap());
    match out.write_all(result.as_bytes()) {
        Ok(_) => Ok(()),
        Err(_) => Err("Error: pwd".to_string()),
    }
}
