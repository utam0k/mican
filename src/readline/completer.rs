use std::env;
use std::collections::HashSet;
use std::fs::read_dir;
use std::path::is_separator;
use std::iter::Iterator;

use readline::terminal;
use readline::color;

#[derive(Default)]
pub struct Completer {
    pub max_len: usize,
    pub w_start: usize,
}

impl Completer {
    pub fn new() -> Self {
        Self {
            max_len: 0,
            w_start: 0,
        }
    }

    pub fn complete(&mut self, path: &str) -> Vec<String> {
        let (_, fname) = match path.rfind(is_separator) {
            Some(pos) => (Some(&path[..pos + 1]), &path[pos + 1..]),
            None => (None, path),
        };

        let env_path = env::var("PATH").unwrap();
        let vec_path: Vec<&str> = env_path.split(':').collect();
        let paths: HashSet<&str> = vec_path.into_iter().collect();
        let mut res: Vec<String> = Vec::new();

        let mut len = 0;

        for p in &paths {
            if let Ok(list) = read_dir(p) {
                for entry in list {
                    if let Ok(entry) = entry {
                        if let Ok(name) = entry.file_name().into_string() {
                            if name.starts_with(fname) {
                                if name.len() > len {
                                    len = name.len();
                                }
                                res.push(name);
                            }
                        }
                    }
                }
            }
        }

        self.max_len = len;
        res
    }

    pub fn create_string(
        &mut self,
        completions: &[String],
        pos: usize,
        start_pos: usize,
        page_size: usize,
    ) -> String {

        let mut line = String::new();
        if completions.len() > page_size * 2 - 1 {
            return line;
        }

        line.push_str(&terminal::move_under_line_first(1));

        let mut w_end = if page_size < completions.len() {
            self.w_start + page_size
        } else {
            completions.len()
        };

        let mut bar_start = 0;
        let mut bar_end = completions.len();

        if completions.len() <= page_size {
            self.w_start = 0;
            w_end = completions.len();
        } else {
            // Move a window edge.
            match pos {
                n if w_end <= n => {
                    let d = pos - page_size;
                    self.w_start = d;
                    w_end = page_size + d;
                }
                n if self.w_start >= pos => {
                    if n == 0 {
                        self.w_start = 0;
                        w_end = page_size;
                    } else {
                        let d = self.w_start - n + 1;
                        self.w_start -= d;
                        w_end -= d;
                    }
                }
                _ => (),
            };

            let blank_n = completions.len() - page_size + 1;
            bar_end = w_end - blank_n;
            bar_start = self.w_start;
        }


        for (i, completion) in completions[self.w_start..w_end].iter().enumerate() {
            line.push_str(&terminal::move_to(start_pos));

            if (pos == 0 && i == 0) || i + self.w_start + 1 == pos {
                line.push_str(&format!("\x1B[7m{}", completion));
            } else {
                line.push_str(&format!("\x1B[44m{}", completion));
            }

            for _ in 0..(self.max_len - completion.len() + 1) {
                line.push_str(" ")
            }

            line.push_str("\x1B[m");

            line.push_str("\x1B[48;5;24m bin \x1B[m");

            if bar_start <= i && i <= bar_end {
                line.push_str(&color::blue(" "));
            } else {
                line.push_str(&color::gray(" "));
            }

            line.push_str("\n");
        }

        line
    }
}
