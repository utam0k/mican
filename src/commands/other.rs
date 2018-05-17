use token::{CommandData, Input, Output};

use std::process::{Command, Stdio};

pub fn run(cmd: CommandData) -> Result<(), String> {
    let mut output = match Command::new(&cmd.program)
        .args(&cmd.options)
        .stdin(match cmd.input.unwrap() {
            Input::Stdin(_) => Stdio::inherit(),
            Input::File(input) => input.into(),
        })
        .stdout(match cmd.out.unwrap() {
            Output::Stdout(_) => Stdio::inherit(),
            Output::File(output) => output.into(),
        })
        .spawn() {
        Ok(p) => p,
        Err(e) => return Err(format!("{}", e)),
    };

    match output.wait() {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{}", e)),
    }
}
