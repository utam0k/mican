
use std::env;
use std::collections::HashSet;
use std::fs::read_dir;
use std::path::is_separator;
use std::iter::Iterator;

use readline::terminal;

#[derive(Default)]
pub struct Completer {
    pub max_len: usize,
}

impl Completer {
    pub fn new() -> Self {
        Self { max_len: 0 }
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

    pub fn to_string(
        &self,
        completions: &[String],
        pos: usize,
        start_pos: usize,
        page_size: usize,
    ) -> String {

        let mut line = String::new();
        if completions.len() >= page_size * 2 - 2 {
            return line;
        }

        line.push_str(&terminal::move_under_line_first(1));

        let mut start: usize = 0;
        let mut end: usize = completions.len();
        let mut d = 0;
        if page_size < completions.len() {
            end = page_size;
        }

        if pos >= page_size {
            d = pos - page_size;
            start = d;
            end = page_size + d;
        }

        for (i, completion) in completions[start..end].iter().enumerate() {
            line.push_str(&terminal::move_to(start_pos));

            if i + 1 + d == pos {
                line.push_str(&format!("\x1B[7m{}", completion));
            } else {
                line.push_str(&format!("\x1B[44m{}", completion));
            }

            for _ in 0..(self.max_len - completion.len() + 1) {
                line.push_str(" ")
            }

            line.push_str("\x1B[m");

            line.push_str("\x1B[48;5;24m bin \x1B[m");


            let mut bar_start = 0;

            let bar_end = if page_size < completions.len() {
                bar_start += d;
                page_size - (completions.len() - page_size) - 1 + d
            } else {
                completions.len()
            };

            let kuhaku_str = "\x1B[48;5;240m \x1B[m";
            let bar_str = "\x1B[44m \x1B[m";

            if bar_start <= i && i <= bar_end {
                line.push_str(bar_str);
            } else {
                line.push_str(kuhaku_str);
            }

            line.push_str("\n");
        }

        line
    }
}
