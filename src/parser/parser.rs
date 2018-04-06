extern crate libc;

use std::fs;
use std::os::unix::io::{RawFd, FromRawFd};
use std::mem;
use std::io::{stdin, stdout};
use std::os::unix::io::AsRawFd;

#[derive(Debug)]
pub struct CommandData {
    pub program: String,
    pub options: Vec<String>,
    pub out: fs::File,
    pub input: fs::File,
}

#[derive(Debug)]
pub enum Token {
    Command(CommandData),
    Pipe,
    // RedirectTo,
}

pub struct Parser {
    pub pos: usize,
    pub input: String,
    pub fds: [RawFd; 2],
}

impl Parser {
    pub fn new(input: String) -> Parser {
        // let in_fd = unsafe { libc::dup(stdin().as_raw_fd()) };
        // let out_fd = unsafe { libc::dup(stdout().as_raw_fd()) };
        let mut fds: [libc::c_int; 2];
        unsafe {
            fds = mem::uninitialized();
            libc::pipe(fds.as_mut_ptr());
        }

        Parser {
            pos: 0,
            input: input,
            // fds: [in_fd, out_fd],
            fds: fds,
        }
    }

    pub fn parse(&mut self) -> Vec<Token> {
        let mut commands: Vec<Token> = vec![];
        loop {
            self.consume_whitespace();
            if self.eof() || self.starts_with("\n") {
                break;
            }
            commands.push(self.parse_token());
        }
        commands
    }

    fn parse_token(&mut self) -> Token {
        match self.next_char() {
            '|' => self.parse_pipe(),
            _ => Token::Command(self.parse_command()),
        }
    }

    fn parse_pipe(&mut self) -> Token {
        let mut fds: [libc::c_int; 2];
        unsafe {
            fds = mem::uninitialized();
            libc::pipe(fds.as_mut_ptr());
        }
        self.fds = [unsafe { libc::dup(self.fds[1]) }, fds[0]];

        self.consume_char();
        Token::Pipe
    }

    fn parse_command(&mut self) -> CommandData {
        let program = self.consume_while(|c| c != ' ');
        let mut options: Vec<String> = vec![];
        loop {
            self.consume_whitespace();
            if self.pipe() {
                break;
            }
            let s = self.consume_while(|c| c != ' ');
            options.push(s);
            if self.eof() {
                break;
            }
        }

        CommandData {
            program: program,
            options: options,
            input: unsafe { fs::File::from_raw_fd(self.fds[0]) },
            out: unsafe { fs::File::from_raw_fd(self.fds[1]) },
        }
    }

    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn pipe(&mut self) -> bool {
        self.starts_with("|")
    }

    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    fn consume_while<F>(&mut self, f: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while !self.eof() && f(self.next_char()) {
            result.push(self.consume_char());
        }
        result
    }

    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        cur_char
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }
}
