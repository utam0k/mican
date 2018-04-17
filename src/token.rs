use std::fs;

#[derive(Debug, PartialEq)]
pub enum Token {
    Command(CommandData),
    Pipe,
    // RedirectTo,
}

#[derive(Debug)]
pub struct CommandData {
    pub program: String,
    pub options: Vec<String>,
    pub input: Option<fs::File>,
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
        self.input = Some(f);
    }
}
