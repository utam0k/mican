extern crate libc;

use std::fs;
use std::io::{stdin, stdout};
use std::mem;
use std::os::unix::io::AsRawFd;
use std::os::unix::io::{FromRawFd, RawFd};

#[derive(Debug)]
pub struct CommandData {
    pub program: String,
    pub options: Vec<String>,
    pub out: Option<fs::File>,
    pub input: Option<fs::File>,
}

impl CommandData {
    pub fn set_out(&mut self, f: fs::File) {
        self.out = Some(f);
    }

    pub fn set_input(&mut self, f: fs::File) {
        self.input = Some(f);
    }
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
    pub pipes: Vec<[RawFd; 2]>,
}

impl Parser {
    pub fn new(input: String) -> Parser {
        Parser {
            pos: 0,
            input: input,
            pipes: vec![],
        }
    }

    pub fn parse(&mut self) -> Vec<CommandData> {
        let mut commands: Vec<Token> = vec![];
        loop {
            self.consume_whitespace();
            if self.eof() || self.starts_with("\n") {
                break;
            }
            commands.push(self.parse_token());
        }
        self.build_pipe(commands)
        // let commands = self.build_pipe(commands);
        // commands
    }

    fn build_pipe(&mut self, commands: Vec<Token>) -> Vec<CommandData> {
        // let mut next_out = unsafe { fs::File::from_raw_fd(libc::dup(stdout().as_raw_fd())) };
        let mut next_in: fs::File =
            unsafe { fs::File::from_raw_fd(libc::dup(stdin().as_raw_fd())) };
        let mut new: Vec<CommandData> = vec![];
        let mut n = 0;
        for cmd in commands {
            match cmd {
                Token::Command(mut c) => {
                    if !self.pipes.is_empty() {
                        let fds = self.pipes.pop().unwrap();
                        let f_in = unsafe { fs::File::from_raw_fd(fds[0]) };
                        let f_out = unsafe { fs::File::from_raw_fd(fds[1]) };
                        if n == 0 {
                            // first
                            c.set_input(unsafe {
                                fs::File::from_raw_fd(libc::dup(stdin().as_raw_fd()))
                            });
                            c.set_out(f_out);
                            next_in = f_in;
                        } else {
                            // middle
                            c.set_input(next_in.try_clone().unwrap());
                            c.set_out(f_out);
                        }
                    } else {
                        // last
                        c.set_input(next_in.try_clone().unwrap());
                        c.set_out(unsafe {
                            fs::File::from_raw_fd(libc::dup(stdout().as_raw_fd()))
                        });
                    }
                    new.push(c);
                    // new.push(cmd);
                }
                Token::Pipe => (),
            }
            n += 1;
        }
        new
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
        self.pipes.push(fds);
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
            input: None,
            out: None,
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
