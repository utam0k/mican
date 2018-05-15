use std::io::{self, Write};

use readline::terminal::unix_terminal;

pub struct Editor {
    pub pos: usize,
    pub prompt: String,
    pub line: String,
}

impl Editor {
    pub fn new(prompt: String) -> Editor {
        Editor {
            pos: 0,
            prompt: prompt,
            line: String::new(),
        }
    }

    pub fn reset(&mut self) {
        self.pos = 0;
        self.line = String::new();
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

    pub fn delete(&mut self, n: usize) -> io::Result<()> {
        if self.is_start() {
            return Ok(());
        }

        let delete_range = self.pos - 1..self.pos + n - 1;
        if let Some(first_tab_index) = self.line[delete_range].find('\t') {
            if let Some(last_tab_index) = self.line.rfind('\t') {
                if first_tab_index == last_tab_index {
                    unix_terminal::move_left(5)?;
                } else {
                    unix_terminal::move_left(7)?;
                }
            }
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

    pub fn come_back(&mut self) -> io::Result<()> {
        if self.pos != 0 {
            let pos = self.pos.clone();
            self.move_to(pos + 1)
        } else {
            self.move_to_first()
        }
    }

    pub fn replace(&mut self, s: &str) -> io::Result<()> {
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
        unix_terminal::clear_to_screen_end()
    }

    pub fn move_left(&mut self, n: usize) -> io::Result<()> {
        if self.is_start() {
            return Ok(());
        }
        self.pos -= n;
        unix_terminal::move_left(n)
    }

    pub fn move_right(&mut self, n: usize) -> io::Result<()> {
        if self.is_last() {
            return Ok(());
        }
        self.pos += n;
        unix_terminal::move_right(n)
    }

    // pub fn move_down(&mut self, n: usize) -> io::Result<()> {
    //     unix_terminal::move_down(n)
    // }

    pub fn move_to_first(&mut self) -> io::Result<()> {
        self.move_to(1)
    }

    pub fn move_to_end(&mut self) -> io::Result<()> {
        let n = self.line.len();
        self.move_to(n + 1)
    }

    fn move_to(&mut self, n: usize) -> io::Result<()> {
        self.pos = n - 1;
        unix_terminal::move_to(self.prompt.len() + n)
    }

    fn is_start(&self) -> bool {
        self.pos < 1
    }

    fn is_last(&self) -> bool {
        self.pos + 1 > self.line.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn setup() -> Editor {
        let mut ed = Editor::new("> ".into());
        ed.put("mican".into()).unwrap();
        ed
    }

    #[test]
    fn test_put() {
        let ed = setup();
        assert_eq!(ed.line, "mican".to_string());
        assert_eq!(ed.pos, "mican".len());
    }

    #[test]
    fn test_delete() {
        let mut ed = setup();
        // TODO ed.delete(2)
        ed.delete(1).unwrap();
        assert_eq!(ed.line, "mica");
        ed.delete(1).unwrap();
        assert_eq!(ed.line, "mic");
    }

    #[test]
    fn test_is_start() {
        let mut ed = Editor::new("> ".into());
        assert!(ed.is_start());
        ed.put("mican".into()).unwrap();
        assert!(!ed.is_start());
        ed.move_to_first().unwrap();
        assert!(ed.is_start());
    }

    #[test]
    fn test_clear_line() {
        let mut ed = setup();
        ed.clear_line().unwrap();
        assert_eq!(ed.pos, "mican".len());
        assert_eq!(ed.line, "".to_string());
    }

    #[test]
    fn test_clear_screen() {
        let mut ed = setup();
        ed.clear_screen().unwrap();
        assert!(ed.is_start());
        assert_eq!(ed.line, "mican".to_string());
    }

    #[test]
    fn test_move_left() {
        let mut ed = setup();
        ed.move_left(1).unwrap();
        assert_eq!(ed.pos, "mican".len() - 1);
        ed.move_left(3).unwrap();
        assert_eq!(ed.pos, "mican".len() - 4);
    }

    #[test]
    fn test_move_right() {
        let mut ed = setup();
        ed.move_right(1).unwrap();
        assert_eq!(ed.pos, "mican".len());
        ed.move_to_first().unwrap();
        ed.move_right(3).unwrap();
        assert_eq!(ed.pos, 3);
    }

    #[test]
    fn test_move_to_first() {
        let mut ed = setup();
        ed.move_to_first().unwrap();
        assert_eq!(ed.pos, 0);
    }

    #[test]
    fn test_move_to_end() {
        let mut ed = setup();
        ed.move_to_end().unwrap();
        assert_eq!(ed.pos, "mican".len());
    }
}
