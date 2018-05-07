use token::CommandData;

use nix::unistd::dup;
use libc::STDOUT_FILENO;

use std::process::Command;
use std::os::unix::io::FromRawFd;
use std::fs;

pub fn run(cmd: CommandData) -> Result<(), String> {
    let input = cmd.input.unwrap();
    let out = cmd.out.unwrap();

    let mut output = match Command::new(&cmd.program)
        .args(&cmd.options)
        .stdin(input)
        .stdout(out)
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
