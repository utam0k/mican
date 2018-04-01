use parser;

use std::env;
use std::fs;

pub fn run(_args: &parser::parser::CommandData) -> Result<(), String> {
    for entry in fs::read_dir(env::current_dir().unwrap()).unwrap() {
        println!("{:?}", entry.unwrap().file_name());
    }
    return Ok(());
}
