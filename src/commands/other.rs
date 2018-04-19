use token::CommandData;

use std::process::Command;

pub fn run(cmd: CommandData) -> Result<(), String> {
    let input = cmd.input.unwrap();
    let out = cmd.out.unwrap();

    let mut output = match Command::new(&cmd.program)
        .args(&cmd.options)
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
