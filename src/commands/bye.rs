use token::CommandData;

use std::io::prelude::*;
use std::process::exit;

use nix::unistd::getppid;
use nix::sys::signal::{kill, Signal};

pub fn run(cmd: CommandData) -> Result<(), String> {
    let result = "Thank you for using Mican\u{1f34a}\n"; // Thank you for using Mican🍊\n
    let mut out = cmd.out.unwrap();
    match out.write_all(result.as_bytes()) {
        Ok(_) => {
            // TODO
            // kill(getppid(), Signal::SIGCHLD).unwrap();
            out.flush().unwrap();
            kill(getppid(), Signal::SIGKILL).unwrap();
            exit(0);
        }
        Err(_) => Err("Error: bye".to_string()),
    }
}
