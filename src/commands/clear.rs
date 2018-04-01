use std::io::{stdout, Write};

pub fn run() -> Result<(), String> {
    stdout().write_all("\x1b[2J\x1b[1;1H".as_bytes()).unwrap();
    return Ok(());
}
