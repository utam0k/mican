use std::env;
use std::collections::HashSet;
use std::fs::read_dir;
use std::path::is_separator;
use std::io;
use std::io::Write;
use std::os::unix::io::AsRawFd;
use std::iter::Iterator;

use readline::terminal::unix_terminal;

pub struct Completer {
    pub result: Vec<String>,
    n: usize,
    height: usize,
    is_after: bool,
}

impl Completer {
    pub fn new() -> Self {
        Completer {
            result: Vec::new(),
            n: 0,
            height: 0,
            is_after: false,
        }
    }

    pub fn clear(&mut self) {
        self.n = 0;
        self.height = 0;
        if self.is_after {
            unix_terminal::move_under_line_first(1).unwrap();
            unix_terminal::clear_to_screen_end().unwrap();
            unix_terminal::move_up(1).unwrap();
        }
        self.is_after = false;
    }

    pub fn complete(&mut self, path: &str) {
        if self.is_after {
            return;
        }

        let (_, fname) = match path.rfind(is_separator) {
            Some(pos) => (Some(&path[..pos + 1]), &path[pos + 1..]),
            None => (None, path),
        };

        let env_path = env::var("PATH").unwrap();
        let vec_path: Vec<&str> = env_path.split(':').collect();
        let paths: HashSet<&str> = vec_path.into_iter().collect();
        let mut result: Vec<String> = Vec::new();

        for p in &paths {
            if let Ok(list) = read_dir(p) {
                for entry in list {
                    if let Ok(entry) = entry {
                        if let Ok(name) = entry.file_name().into_string() {
                            if name.starts_with(fname) {
                                result.push(name);
                            }
                        }
                    }
                }
            }
        }
        self.height = result.join(" ").len() /
            unix_terminal::get_winsize(io::stdout().as_raw_fd())
                .unwrap()
                .ws_col as usize + 1;

        self.result = result.clone();
        self.is_after = true;
    }

    pub fn show(&mut self) -> io::Result<()> {
        let stdout = io::stdout();
        let mut lock = stdout.lock();

        unix_terminal::move_under_line_first(1)?;

        let mut line = String::new();
        for (i, completion) in self.result.clone().iter().enumerate() {
            if i + 1 == self.n {
                line.push_str(&format!("\x1B[7m{}\x1B[0m ", completion));
            } else {
                line.push_str(&format!("{} ", completion));
            }
        }

        lock.write_all(&line.as_bytes())?;
        lock.flush()?;

        unix_terminal::move_up(self.height)
    }

    pub fn next(&mut self) -> Option<&String> {
        if self.result.is_empty() {
            return None;
        }
        let cmd = self.result.get(self.n);
        if cmd.is_none() {
            self.n = 1;
            return self.result.get(0);
        }
        self.n += 1;
        return cmd;
    }

    pub fn is_empty(&self) -> bool {
        return self.result.is_empty();
    }
}
