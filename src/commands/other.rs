use std::process::Command;
use std::fs::File;

use parser;

pub fn run(cmd: parser::parser::CommandData) -> Result<(), String> {
    let mut input = cmd.input.try_clone().unwrap();
    let out = cmd.out.try_clone().unwrap();
    println!("input: {:?}, out: {:?}", input, out);

    let mut contents = String::new();
    use std::io::prelude::*;
    input.read_to_string(&mut contents).unwrap();
    println!("input: {:?}", contents);

    let mut output = match Command::new(&cmd.program)
        .args(&cmd.options)
        .stdin(input)
        .stdout(out)
        .spawn() {
        Ok(p) => p,
        Err(e) => return Err(format!("{}", e)),
    };

    println!("other run");
    match output.wait() {
        Ok(_) => return Ok(()),
        Err(e) => return Err(format!("{}", e)),
    }
}
