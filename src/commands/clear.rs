use token::CommandData;

use std::io::Write;

pub fn run(cmd: CommandData) -> Result<(), String> {
    let mut out = cmd.out.unwrap();
    out.write_all("\x1b[2J\x1b[1;1H".as_bytes()).unwrap();
    return Ok(());
}
