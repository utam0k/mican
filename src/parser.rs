#[derive(Debug)]
pub struct Command {
    command: String,
    options: Vec<String>,
}

#[derive(Debug)]
pub enum Token {
    Command {
        command: Command,
    },
    Pipe,
    // RedirectTo,
}

pub struct Parser {
    pub pos: usize,
    pub input: String,
}

impl Parser {
    pub fn new(input: String) -> Parser {
        Parser { 
            pos: 0,
            input: input, 
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
            '|' => Token::Pipe,
            _   => Token::Command{ command: self.parse_command()},
        }
    }

    fn parse_command(&mut self) -> Command {
        let command = self.consume_while(|c| c != ' ');
        let mut options: Vec<String> = vec![];
        loop {
            self.consume_whitespace();
            let s = self.consume_while(|c| c != ' ');
            options.push(s);
            if self.eof() || self.starts_with("|") {
                break;
            }
        }
        Command {command: command, options: options}
    }

    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    fn consume_while<F>(&mut self, f: F) -> String
        where F: Fn(char) -> bool {
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

