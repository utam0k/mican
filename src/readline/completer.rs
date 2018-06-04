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
    pub completion_area_first: usize,
}

impl Completer {
    pub fn new() -> Self {
        Self {
            max_len: 0,
            completion_area_first: 0,
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

    pub fn create_completion_area(
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

        let is_needed_scroll = page_size < completions.len();

        let mut scroll_bar_start = 0;
        let mut scroll_bar_end = completions.len();

        let create_range = |start: usize| start..(start + page_size);
        let mut completion_area_range = create_range(self.completion_area_first);

        if is_needed_scroll {
            let is_overed = pos >= self.completion_area_first + page_size;
            let is_undered = pos < self.completion_area_first;
            let is_head = pos == self.completion_area_first;
            // Move a window edge.
            if is_overed {
                let exceeded_n = pos - page_size;
                self.completion_area_first = exceeded_n;
                completion_area_range = create_range(exceeded_n);
            } else if is_undered {
                let fall_n = self.completion_area_first - pos + 1;
                self.completion_area_first -= fall_n;
                completion_area_range = create_range(self.completion_area_first);
            } else if is_head {
                self.completion_area_first = 0;
                completion_area_range = create_range(0);
            }

            let blank_n = completions.len() - page_size + 1;
            scroll_bar_start = self.completion_area_first;
            scroll_bar_end = self.completion_area_first + page_size - blank_n;
        } else {
            // TODO
            self.completion_area_first = 0;
            completion_area_range = 0..completions.len();
        }

        for (i, completion) in completions[completion_area_range].iter().enumerate() {
            line.push_str(&terminal::move_to(start_pos));

            let padded_completion = format!("{:width$}", completion, width = self.max_len + 1);

            if (pos == 0 && i == 0) || i + self.completion_area_first + 1 == pos {
                line.push_str(&color::white(padded_completion.as_ref()));
            } else {
                line.push_str(&color::light_blue(padded_completion.as_ref()));
            }

            line.push_str(&color::dark_blue(" bin "));

            if scroll_bar_start <= i && i <= scroll_bar_end {
                line.push_str(&color::gray(" "));
            } else {
                line.push_str(&color::light_blue(" "));
            }

            line.push_str("\n");
        }

        line
    }
}
