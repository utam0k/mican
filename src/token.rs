use std::fs;
use std::io::{self, Read, Write};

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

#[derive(Debug)]
pub enum Output {
    File(fs::File),
    Stdout(io::Stdout),
}

impl From<io::Stdout> for Output {
    fn from(output: io::Stdout) -> Self {
        Output::Stdout(output)
    }
}

impl From<fs::File> for Output {
    fn from(output: fs::File) -> Self {
        Output::File(output)
    }
}

impl Write for Output {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match *self {
            Output::File(ref mut file) => file.write(buf),
            Output::Stdout(ref mut stdout) => stdout.write(buf),
        }
    }

    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        match *self {
            Output::File(ref mut file) => file.write_all(buf),
            Output::Stdout(ref mut stdout) => stdout.write_all(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match *self {
            Output::File(ref mut file) => file.flush(),
            Output::Stdout(ref mut stdout) => stdout.flush(),
        }
    }
}

impl Clone for Output {
    fn clone(&self) -> Output {
        match *self {
            Output::File(ref file) => Output::File(file.try_clone().unwrap()),
            Output::Stdout(_) => Output::Stdout(io::stdout()),
        }
    }
}

#[derive(Debug)]
pub struct CommandData {
    pub program: String,
    pub options: Vec<String>,
    pub input: Option<Input>,
    pub out: Option<Output>,
}

impl PartialEq for CommandData {
    fn eq(&self, other: &CommandData) -> bool {
        self.program == other.program && self.options == other.options
    }
}

impl CommandData {
    pub fn set_out<T: Into<Output>>(&mut self, output: T) {
        self.out = Some(output.into());
    }

    pub fn set_input<T: Into<Input>>(&mut self, f: T) {
        self.input = Some(f.into());
    }
}
