use libc;

use token::{CommandData, Token};

use std::fs;
use std::io::{stdin, stdout};
use std::mem;
use std::os::unix::io::AsRawFd;
use std::os::unix::io::{FromRawFd, RawFd};

const PIPE: char = '|';

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
        let tokens = self.parse_tokens();
        self.build_pipe(tokens)
    }

    pub fn parse_tokens(&mut self) -> Vec<Token> {
        let mut commands: Vec<Token> = vec![];
        loop {
            self.consume_whitespace();
            if self.eof() || self.starts_with("\n") {
                return commands;
            }
            commands.push(self.parse_token());
        }
    }

    fn build_pipe(&mut self, mut commands: Vec<Token>) -> Vec<CommandData> {
        commands.reverse();
        self.set_pipe(
            unsafe { fs::File::from_raw_fd(libc::dup(stdin().as_raw_fd())) },
            commands,
            vec![],
        )
    }

    fn set_pipe(
        &mut self,
        next_in: fs::File,
        mut commands: Vec<Token>,
        mut new: Vec<CommandData>,
    ) -> Vec<CommandData> {
        if commands.is_empty() {
            return new;
        }

        match commands.pop().unwrap() {
            Token::Command(mut c) => {
                if !self.pipes.is_empty() {
                    let fds = self.pipes.pop().unwrap();
                    let f_in = unsafe { fs::File::from_raw_fd(fds[0]) };
                    let f_out = unsafe { fs::File::from_raw_fd(fds[1]) };
                    if new.is_empty() {
                        // first
                        c.set_input(unsafe {
                            fs::File::from_raw_fd(libc::dup(stdin().as_raw_fd()))
                        });
                    } else {
                        // middle
                        c.set_input(next_in.try_clone().unwrap());
                    }
                    c.set_out(f_out);
                    new.push(c);
                    self.set_pipe(f_in, commands, new)
                } else {
                    // last
                    c.set_input(next_in.try_clone().unwrap());
                    c.set_out(unsafe { fs::File::from_raw_fd(libc::dup(stdout().as_raw_fd())) });
                    new.push(c);
                    self.set_pipe(next_in, commands, new)
                }
            }
            Token::Pipe => self.set_pipe(next_in, commands, new),
        }
    }

    fn parse_token(&mut self) -> Token {
        match self.next_char() {
            PIPE => self.parse_pipe(),
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
        let program = self.consume_space_or_pipe();
        let mut options: Vec<String> = vec![];
        loop {
            self.consume_whitespace();
            if self.pipe() {
                break;
            }
            let s = self.consume_space_or_pipe();
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

    fn consume_space_or_pipe(&mut self) -> String {
        self.consume_while(|c| !char::is_whitespace(c) && c != PIPE)
    }
}

#[test]
fn test_parse_tokens() {
    let input = "ls -al | grep main.rs".to_string();
    let result = Parser::new(input).parse_tokens();
    let ls = Token::Command(CommandData {
        program: "ls".to_string(),
        options: vec!["-al".to_string()],
        input: None,
        out: None,
    });
    let grep = Token::Command(CommandData {
        program: "grep".to_string(),
        options: vec!["main.rs".to_string()],
        input: None,
        out: None,
    });

    assert_eq!(result, vec![ls, Token::Pipe, grep]);
}
