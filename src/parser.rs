#[derive(Debug)]
pub struct CommandData {
    pub program: String,
    pub options: Vec<String>,
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
            '|' => self.parse_pipe(),
            _ => Token::Command(self.parse_command()),
        }
    }

    fn parse_pipe(&mut self) -> Token {
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
