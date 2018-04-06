use parser;
use std::env;
use std::io::prelude::*;

pub fn run(mut cmd: parser::parser::CommandData) -> Result<(), String> {
    let result = format!("{:?}\n", env::current_dir().unwrap());
    let mut out = cmd.out.unwrap();
    match out.write_all(result.as_bytes()) {
        Ok(_) => {
            println!("{:?}", out);
            Ok(())
        }
        Err(_) => Err("Error: pwd".to_string()),
    }
}
