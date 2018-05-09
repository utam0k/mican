use token::{CommandData, Input, Output};

use nix::unistd::dup;
use libc::STDOUT_FILENO;

use std::process::{Command, Stdio};
use std::os::unix::io::FromRawFd;
use std::fs;

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
        .stderr(unsafe {
            fs::File::from_raw_fd(dup(STDOUT_FILENO).unwrap())
        })
        .spawn() {
        Ok(p) => p,
        Err(e) => return Err(format!("{}", e)),
    };

    match output.wait() {
        Ok(_) => return Ok(()),
        Err(e) => return Err(format!("{}", e)),
    }
}
