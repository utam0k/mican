use std::io::{self, Write};

pub struct Term {
    pub pos: usize,
    pub prompt: String,
    pub line: String,
}

impl Term {
    pub fn new(prompt: String) -> Term {
        Term {
            pos: 0,
            prompt: prompt,
            line: String::new(),
        }
    }

    pub fn reset(&mut self) {
        self.pos = 0;
        self.line = String::new();
    }

    pub fn delete(&mut self, n: usize) -> io::Result<()> {
        if self.is_start() {
            return Ok(());
        }
        self.line.remove(self.pos - n);
        self.move_left(n)?;
        self.clear_to_screen_end()?;
        if !self.is_last() {
            let line = self.line.clone();
            let pos = self.pos;
            self.write(&line.get(pos..).unwrap())?;
            return self.move_to(pos + n);
        } else {
            return Ok(());
        }
    }

    pub fn put(&mut self, s: String) -> io::Result<()> {
        if !self.is_last() {
            self.line.insert_str(self.pos, &s);
            let line = self.line.clone();
            let old_pos = self.pos;
            self.clear_to_screen_end()?;
            self.write(&line.get(old_pos..).unwrap())?;
            return self.move_to(old_pos + s.len() + 1);
        } else {
            self.line.insert_str(self.pos, &s);
            return self.write_str(&s);
        }
    }

    pub fn write_line(&mut self) -> io::Result<()> {
        let s = &self.line;
        self.write(s)
    }

    pub fn write_prompt(&mut self) -> io::Result<()> {
        self.write(&self.prompt)
    }

    pub fn write_str(&mut self, s: &str) -> io::Result<()> {
        self.pos += s.len();
        self.write(s)
    }

    pub fn rewrite(&mut self, s: &str) -> io::Result<()> {
        self.clear_line().unwrap();
        self.line = s.to_string();
        self.write_line()
    }

    pub fn new_line(&mut self) -> io::Result<()> {
        self.write("\n")
    }

    fn write(&self, s: &str) -> io::Result<()> {
        let stdout = io::stdout();
        let mut lock = stdout.lock();

        lock.write_all(s.as_bytes())?;
        lock.flush()
    }

    pub fn clear_line(&mut self) -> io::Result<()> {
        self.line = String::new();
        let old_pos = self.pos;
        self.move_to_first()?;
        self.clear_to_screen_end()?;
        self.pos = old_pos;
        return Ok(());
    }


    pub fn clear_screen(&mut self) -> io::Result<()> {
        self.pos = 0;
        self.write(&format!("\x1b[2J\x1b[1;1H{}", self.prompt))
    }

    pub fn clear_to_screen_end(&self) -> io::Result<()> {
        self.write("\x1b[J")
    }

    pub fn move_left(&mut self, n: usize) -> io::Result<()> {
        if self.is_start() {
            return Ok(());
        }
        self.pos -= n;
        self.write(&format!("\x1b[{}D", n))
    }

    pub fn move_right(&mut self, n: usize) -> io::Result<()> {
        if self.is_last() {
            return Ok(());
        }
        self.pos += n;
        self.write(&format!("\x1b[{}C", n))
    }

    fn move_to_first(&mut self) -> io::Result<()> {
        self.move_to(1)
    }

    fn move_to(&mut self, n: usize) -> io::Result<()> {
        self.pos = n - 1;
        self.write(&format!("\x1b[{}G", self.prompt.len() + n))
    }

    fn is_start(&self) -> bool {
        self.pos < 1
    }

    fn is_last(&self) -> bool {
        self.pos + 1 > self.line.len()
    }
}
