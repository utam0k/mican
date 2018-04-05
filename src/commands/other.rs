use std::process::Command;
use std::fs;

use parser;

pub fn run(
    command: &parser::parser::CommandData,
    input: &fs::File,
    out: &fs::File,
) -> Result<(), String> {
    let input = input.try_clone().unwrap();
    let out = out.try_clone().unwrap();
    let mut output = match Command::new(&command.program)
        .args(&command.options)
        .stdin(input)
        .stdout(out)
        .spawn() {
        Ok(p) => p,
        Err(e) => return Err(format!("{}", e)),
    };

    match output.wait() {
        Ok(_) => return Ok(()),
        Err(e) => return Err(format!("{}", e)),
    }
}
