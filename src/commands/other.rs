use std::process::{Command, Stdio};

use parser;

pub fn run(command: &parser::parser::CommandData) -> Result<(), String> {
    let mut output = match Command::new(&command.program)
        .args(&command.options)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn() {
        Ok(p) => p,
        Err(e) => return Err(format!("{}", e)),
    };

    match output.wait() {
        Ok(_) => return Ok(()),
        Err(e) => return Err(format!("{}", e)),
    }
}
