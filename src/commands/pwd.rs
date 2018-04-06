use std::env;
use std::io::prelude::*;
use parser;

pub fn run(mut cmd: parser::parser::CommandData) -> Result<(), String> {
    let result = format!("{:?}\n", env::current_dir().unwrap());
    match cmd.out.write_all(result.as_bytes()) {
        Ok(_) => {
            println!("{:?}", cmd.out);
            Ok(())
        }
        Err(_) => Err("Error: pwd".to_string()),
    }
}
