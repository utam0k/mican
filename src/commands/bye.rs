use parser;
use std::io::prelude::*;
use std::process::exit;

pub fn run(cmd: parser::CommandData) -> Result<(), String> {
    let result = "Thank you for using Mican🍊\n";
    let mut out = cmd.out.unwrap();
    match out.write_all(result.as_bytes()) {
        Ok(_) => {
            exit(1);
        }
        Err(_) => Err("Error: bye".to_string()),
    }
}
