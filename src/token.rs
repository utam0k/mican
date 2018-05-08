use std::fs;
use std::io::{self, Read};

#[derive(Debug, PartialEq)]
pub enum Token {
    Command(CommandData),
    Pipe,
    // RedirectTo,
}

#[derive(Debug)]
pub enum Input {
    File(fs::File),
    Stdin(io::Stdin),
}

impl Read for Input {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match *self {
            Input::File(ref mut file) => file.read(buf),
            Input::Stdin(ref mut stdin) => stdin.read(buf),
        }
    }
}

#[derive(Debug)]
pub struct CommandData {
    pub program: String,
    pub options: Vec<String>,
    pub input: Option<Input>,
    pub out: Option<fs::File>,
}

impl PartialEq for CommandData {
    fn eq(&self, other: &CommandData) -> bool {
        self.program == other.program && self.options == other.options
    }
}

impl CommandData {
    pub fn set_out(&mut self, f: fs::File) {
        self.out = Some(f);
    }

    pub fn set_input(&mut self, f: fs::File) {
        self.input = Some(Input::File(f));
    }
}
