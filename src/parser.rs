use nix::unistd::pipe;

use token::{CommandData, Token, Input};

use std::fs;
use std::os::unix::io::{FromRawFd, RawFd};
use std::io;

const PIPE: char = '|';

pub struct Parser {
    pub pos: usize,
    pub input: String,
    pub pipes: Vec<(RawFd, RawFd)>,
}

impl Parser {
    pub fn new(input_: String) -> Self {
        Self {
            pos: 0,
            input: input_,
            pipes: vec![],
        }
    }

    pub fn parse(&mut self) -> Vec<CommandData> {
        let tokens = self.parse_tokens();
        self.build_pipes(tokens)
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

    fn build_pipes(&mut self, mut commands: Vec<Token>) -> Vec<CommandData> {
        commands.reverse();
        self.set_pipe(Input::Stdin(io::stdin()), commands)
    }

    fn set_pipe(&mut self, next_in: Input, mut commands: Vec<Token>) -> Vec<CommandData> {
        if commands.is_empty() {
            return Vec::new();
        }

        match commands.pop().unwrap() {
            Token::Command(mut c) => {
                if self.pipes.is_empty() {
                    c.set_input(next_in.clone());
                    c.set_out(io::stdout());
                    let mut ini = vec![c];
                    ini.append(&mut self.set_pipe(next_in, commands));
                    ini
                } else {
                    let fds = self.pipes.pop().unwrap();
                    let f_in = unsafe { fs::File::from_raw_fd(fds.0) };
                    let f_out = unsafe { fs::File::from_raw_fd(fds.1) };
                    c.set_input(next_in);
                    c.set_out(f_out);
                    let mut ini = vec![c];
                    ini.append(&mut self.set_pipe(Input::File(f_in), commands));
                    ini
                }
            }
            Token::Pipe => self.set_pipe(next_in, commands),
        }
    }

    fn parse_token(&mut self) -> Token {
        match self.next_char() {
            PIPE => self.parse_pipe(),
            _ => Token::Command(self.parse_command()),
        }
    }

    fn parse_pipe(&mut self) -> Token {
        let fds = pipe().unwrap();
        self.pipes.push(fds);
        self.consume_char();
        Token::Pipe
    }

    fn parse_command(&mut self) -> CommandData {
        let program_ = self.consume_space_or_pipe();
        let mut options_: Vec<String> = vec![];
        loop {
            self.consume_whitespace();
            if self.pipe() {
                break;
            }
            let s = self.consume_space_or_pipe();
            options_.push(s);
            if self.eof() {
                break;
            }
        }

        CommandData {
            program: program_,
            options: options_,
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
