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

impl From<io::Stdin> for Input {
    fn from(input: io::Stdin) -> Self {
        Input::Stdin(input)
    }
}

impl From<fs::File> for Input {
    fn from(input: fs::File) -> Self {
        Input::File(input)
    }
}

impl Read for Input {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match *self {
            Input::File(ref mut file) => file.read(buf),
            Input::Stdin(ref mut stdin) => stdin.read(buf),
        }
    }
}

impl Clone for Input {
    fn clone(&self) -> Input {
        match *self {
            Input::File(ref file) => Input::File(file.try_clone().unwrap()),
            Input::Stdin(_) => Input::Stdin(io::stdin()),
        }
    }
}

impl Input {
    pub fn try_clone(&self) -> io::Result<Input> {
        match *self {
            Input::File(ref file) => Ok(Input::File(file.try_clone()?)),
            Input::Stdin(_) => Ok(Input::Stdin(io::stdin())),
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

    pub fn set_input<T: Into<Input>>(&mut self, f: T) {
        self.input = Some(f.into());
    }
}
