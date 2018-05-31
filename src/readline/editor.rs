use std::io::{self, Write};
use std::rc::Rc;

use nix::libc::STDOUT_FILENO;

use readline::terminal;
use readline::completer::Completer;
use readline::history::History;
use readline::buffer::Buffer;

pub struct Editor {
    pub pos: usize,
    pub prompt: String,
    pub buffer: Buffer,
    pub w_buffer: String,

    pub win_size: terminal::Winsize,

    pub completer: Completer,
    pub completions: Rc<Vec<String>>,
    pub completer_index: usize,
    pub completer_is_after: bool,

    pub history: History,
}

pub trait Complete {
    fn complete(&mut self);

    fn completion_disply(&mut self);

    fn completion_clear(&mut self);

    fn completion_next(&mut self);

    fn completion_prev(&mut self);
}

impl Complete for Editor {
    fn complete(&mut self) {
        if self.completer_is_after {
            return;
        }
        self.completer_is_after = true;
        let mut complitions = self.completer.complete(&self.buffer.as_str());
        complitions.sort();
        self.completions = Rc::new(complitions);
    }

    fn completion_clear(&mut self) {
        if !self.completer_is_after {
            return;
        }

        if self.completer_is_after {
            self.w_buffer.push_str(&terminal::move_under_line_first(1));
            self.w_buffer.push_str(&terminal::clear_to_screen_end());
            self.w_buffer.push_str(&terminal::move_up(1));
            self.come_back();
        }

        self.completer_index = 0;
        self.completer_is_after = false;
    }

    fn completion_disply(&mut self) {
        //
        let page_size = 10;
        let height = if self.completions.len() < page_size {
            self.completions.len() + 1
        } else {
            page_size + 1
        };

        let completions = self.completer.create_string(
            &self.completions,
            self.completer_index,
            self.prompt.len() + 1,
            // self.pos,
            page_size,
        );
        self.write_sub(&completions, height);
    }

    fn completion_next(&mut self) {
        if self.completer_is_after {
            self.completer_index += 1;
            let index = if self.completer_index > self.completions.len() {
                self.completer_index = 0;
                0
            } else {
                self.completer_index
            };

            if let Some(cmd) = self.completions.clone().get(index) {
                self.replace(&cmd);
                self.move_to_end();
            }
        }
        self.completion_disply();
    }

    fn completion_prev(&mut self) {
        if self.completer_is_after {
            if self.completer_index <= 1 {
                self.completer_index = self.completions.len();
            } else {
                self.completer_index -= 1;
            }

            let index = if self.completer_index > self.completions.len() {
                self.completer_index = 1;
                1
            } else {
                self.completer_index
            };

            if let Some(cmd) = self.completions.clone().get(index - 1) {
                self.replace(&cmd);
                self.move_to_end();
            }
        }
        self.completion_disply();
    }
}

impl Editor {
    pub fn new(prompt_: String) -> Self {
        Self {
            pos: 0,
            prompt: prompt_,
            buffer: Buffer::new(),
            w_buffer: String::new(),

            win_size: terminal::get_winsize(STDOUT_FILENO).unwrap(),

            completer: Completer::new(),
            completions: Rc::new(Vec::new()),
            completer_index: 0,
            completer_is_after: false,

            history: History::new(),
        }
    }

    pub fn reset(&mut self) {
        self.pos = 0;
        self.buffer = Buffer::new();
    }

    pub fn line(&self) -> String {
        self.buffer.as_str()
    }

    pub fn put(&mut self, s: &str) {
        if self.is_last() {
            self.buffer.insert_str(self.pos, &s);
            self.write_str(&s);
        } else {
            self.buffer.insert_str(self.pos, &s);
            let line = self.buffer.clone();
            let old_pos = self.pos;
            self.clear_to_screen_end();
            self.w_buffer.push_str(
                &line.as_str().get(old_pos..).unwrap(),
            );
            self.move_to(old_pos + s.len() + 1);
        }
    }

    pub fn delete(&mut self, n: usize) {
        if self.is_start() {
            return;
        }

        let delete_range = self.pos - 1..self.pos + n - 1;
        if let Some(first_tab_index) = self.buffer.as_str()[delete_range].find('\t') {
            if let Some(last_tab_index) = self.buffer.as_str().rfind('\t') {
                if first_tab_index == last_tab_index {
                    self.w_buffer.push_str(&terminal::move_left(5));
                } else {
                    self.w_buffer.push_str(&terminal::move_left(7));
                }
            }
        }

        self.buffer.remove(self.pos - n);
        self.move_left(n);
        self.clear_to_screen_end();
        if !self.is_last() {
            let line = self.buffer.clone();
            let pos = self.pos;
            self.w_buffer.push_str(&line.as_str().get(pos..).unwrap());
            self.move_to(pos + n);
        }
    }

    pub fn write_line(&mut self) {
        self.w_buffer.push_str(&self.buffer.as_str());
    }

    pub fn write_prompt(&mut self) {
        self.w_buffer.push_str(&self.prompt);
    }

    pub fn write_str(&mut self, s: &str) {
        self.pos += s.len();
        self.w_buffer.push_str(s)
    }

    pub fn come_back(&mut self) {
        if self.pos == 0 {
            self.move_to_first();
        } else {
            let pos = self.pos;
            self.move_to(pos + 1);
        }
    }

    pub fn replace(&mut self, s: &str) {
        self.clear_line().unwrap();
        self.buffer = Buffer::from(s);
        self.write_line();
    }

    pub fn new_line(&mut self) {
        self.w_buffer.push_str("\n");
    }

    pub fn clear_line(&mut self) -> io::Result<()> {
        self.buffer = Buffer::new();
        let old_pos = self.pos;
        self.move_to_first();
        self.clear_to_screen_end();
        self.pos = old_pos;
        Ok(())
    }


    pub fn clear_screen(&mut self) {
        self.pos = 0;
        self.w_buffer.push_str(
            &format!("\x1b[2J\x1b[1;1H{}", self.prompt),
        );
    }

    pub fn clear_to_screen_end(&mut self) {
        self.w_buffer.push_str(&terminal::clear_to_screen_end());
    }

    pub fn move_left(&mut self, n: usize) {
        if self.is_start() {
            return;
        }
        self.pos -= n;
        self.w_buffer.push_str(&terminal::move_left(n));
    }

    pub fn move_right(&mut self, n: usize) {
        if self.is_last() {
            return;
        }
        self.pos += n;
        self.w_buffer.push_str(&terminal::move_right(n));
    }

    pub fn move_to_first(&mut self) {
        self.move_to(1);
    }

    pub fn move_to_end(&mut self) {
        let n = self.buffer.len();
        self.move_to(n + 1);
    }

    fn move_to(&mut self, n: usize) {
        self.pos = n - 1;
        self.w_buffer.push_str(
            &terminal::move_to(self.prompt.len() + n),
        );
    }

    fn is_start(&self) -> bool {
        self.pos < 1
    }

    fn is_last(&self) -> bool {
        self.pos + 1 > self.buffer.len()
    }

    pub fn display(&mut self) -> io::Result<()> {
        self.write(&self.w_buffer)?;
        self.w_buffer.clear();
        Ok(())
    }

    pub fn write_sub(&mut self, s: &str, height: usize) {
        self.w_buffer.push_str(s);
        self.w_buffer.push_str(&terminal::move_up(height));
        self.move_to_end();
    }

    fn write(&self, s: &str) -> io::Result<()> {
        let stdout = io::stdout();
        let mut lock = stdout.lock();

        write!(lock, "{}", s)?;
        lock.flush()
    }
}
#[cfg(test)]
mod test {
    use super::*;

    fn setup() -> Editor {
        let mut ed = Editor::new("> ".into());
        ed.put("mican");
        ed
    }

    #[test]
    fn test_put() {
        let ed = setup();
        assert_eq!(ed.line(), "mican");
        assert_eq!(ed.pos, "mican".len());
    }

    #[test]
    fn test_delete() {
        let mut ed = setup();
        // TODO ed.delete(2)
        ed.delete(1);
        assert_eq!(ed.line(), "mica");
        ed.delete(1);
        assert_eq!(ed.line(), "mic");
    }

    #[test]
    fn test_is_start() {
        let mut ed = Editor::new("> ".into());
        assert!(ed.is_start());
        ed.put("mican");
        assert!(!ed.is_start());
        ed.move_to_first();
        assert!(ed.is_start());
    }

    #[test]
    fn test_clear_line() {
        let mut ed = setup();
        ed.clear_line().unwrap();
        assert_eq!(ed.pos, "mican".len());
        assert_eq!(ed.line(), "");
    }

    #[test]
    fn test_clear_screen() {
        let mut ed = setup();
        ed.clear_screen();
        assert!(ed.is_start());
        assert_eq!(ed.line(), "mican");
    }

    #[test]
    fn test_move_left() {
        let mut ed = setup();
        ed.move_left(1);
        assert_eq!(ed.pos, "mican".len() - 1);
        ed.move_left(3);
        assert_eq!(ed.pos, "mican".len() - 4);
    }

    #[test]
    fn test_move_right() {
        let mut ed = setup();
        ed.move_right(1);
        assert_eq!(ed.pos, "mican".len());
        ed.move_to_first();
        ed.move_right(3);
        assert_eq!(ed.pos, 3);
    }

    #[test]
    fn test_move_to_first() {
        let mut ed = setup();
        ed.move_to_first();
        assert_eq!(ed.pos, 0);
    }

    #[test]
    fn test_move_to_end() {
        let mut ed = setup();
        ed.move_to_end();
        assert_eq!(ed.pos, "mican".len());
    }
}
