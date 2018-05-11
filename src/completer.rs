use std::env;
use std::collections::HashSet;
use std::fs::read_dir;
use std::path::is_separator;
use std::io;

use term::Term;

pub struct Completer {
    term: Term,
    result: Vec<String>,
}

impl Completer {
    pub fn new() -> Self {
        Completer {
            term: Term::new("".into()),
            result: Vec::new(),
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
        self.result = result.clone();
        result
    }

    pub fn show(&mut self) -> io::Result<()> {
        self.term.new_line()?;
        self.term.write_str(&self.result.join(" "))?;
        self.term.write_str("\x1b[1A")
    }
}
