use std::env;
use std::collections::HashSet;
use std::fs::read_dir;
use std::path::is_separator;
use std::io;

use term::Term;
use cursor::unix_cursor;

pub struct Completer {
    term: Term,
    pub result: Option<Vec<String>>,
    n: usize,
}

impl Completer {
    pub fn new() -> Self {
        Completer {
            term: Term::new("".into()),
            result: None,
            n: 0,
        }
    }

    pub fn clear(&mut self) {
        self.n = 0;
        if self.result.is_some() {
            unix_cursor::move_under_line_first(1).unwrap();
            self.term.clear_to_screen_end().unwrap();
            // TODO
            unix_cursor::move_up(1).unwrap();
        }
        self.result = None;
    }

    pub fn complete(&mut self, path: &str) {
        if self.result.is_some() {
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
        self.result = Some(result.clone());
    }

    pub fn show(&mut self) -> io::Result<()> {
        self.term.new_line()?;
        let mut line = String::new();
        for (i, completion) in self.result.clone().unwrap().iter().enumerate() {
            if i + 1 == self.n {
                line.push_str(&format!("\x1B[7m{}\x1B[0m ", completion));
            } else {
                line.push_str(&format!("{} ", completion));
            }
        }

        self.term.write_str(&line)?;
        // TODO
        self.term.move_down(1)
    }

    pub fn next(&mut self) -> Option<&String> {
        if let Some(ref mut vec) = self.result {
            let cmd = vec.get(self.n);
            if cmd.is_none() {
                self.n = 1;
                return vec.get(0);
            }
            self.n += 1;
            return cmd;
        }
        return None;
    }

    pub fn is_empty(&self) -> bool {
        return self.result.is_none();
    }
}
