use std::iter::FromIterator;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorPosition {
    InWord(usize),

    OnWordLeftEdge(usize),

    OnWordRightEdge(usize),

    InSpace(Option<usize>, Option<usize>),
}

impl CursorPosition {
    pub fn get(cursor: usize, words: &[(usize, usize)]) -> Self {
        if words.is_empty() {
            return CursorPosition::InSpace(None, None);
        } else if cursor == words[0].0 {
            return CursorPosition::OnWordLeftEdge(0);
        } else if cursor < words[0].0 {
            return CursorPosition::InSpace(None, Some(0));
        }

        for (i, &(start, end)) in words.iter().enumerate() {
            if start == cursor {
                return CursorPosition::OnWordLeftEdge(i);
            } else if end == cursor {
                return CursorPosition::OnWordRightEdge(i);
            } else if start < cursor && cursor < end {
                return CursorPosition::InWord(i);
            } else if cursor < start {
                return CursorPosition::InSpace(Some(i - 1), Some(i));
            }
        }

        CursorPosition::InSpace(Some(words.len() - 1), None)
    }
}

/// A buffer for text in the line editor.
#[derive(Clone, Default)]
pub struct Buffer {
    data: Vec<char>,
}

impl FromIterator<char> for Buffer {
    fn from_iter<T: IntoIterator<Item = char>>(t: T) -> Self {
        Self { data: t.into_iter().collect() }
    }
}

impl From<Buffer> for String {
    fn from(buf: Buffer) -> Self {
        Self::from_iter(buf.data)
    }
}

impl From<String> for Buffer {
    fn from(s: String) -> Self {
        Self::from_iter(s.chars())
    }
}

impl<'a> From<&'a str> for Buffer {
    fn from(s: &'a str) -> Self {
        Self::from_iter(s.chars())
    }
}

impl Buffer {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    // TODO
    pub fn as_str(&self) -> String {
        String::from(self.clone())
    }

    pub fn remove(&mut self, start: usize, end: usize) -> Vec<char> {
        self.data.drain(start..end).collect()
    }

    pub fn insert_str(&mut self, idx: usize, string: &str) {
        for (i, c) in string.chars().enumerate() {
            self.data.insert(idx + i, c)
        }
    }

    pub fn get_words(&self) -> Vec<(usize, usize)> {
        let mut res = Vec::new();

        let mut word_start = None;
        let mut is_backslash = false;

        for (i, &c) in self.data.iter().enumerate() {
            if c == '\\' {
                is_backslash = true;
                continue;
            }

            if let Some(start) = word_start {
                if c == ' ' && !is_backslash {
                    res.push((start, i));
                    word_start = None;
                }
            } else if c != ' ' {
                word_start = Some(i);
            }

            is_backslash = false;
        }

        if let Some(start) = word_start {
            res.push((start, self.len()));
        }

        res
    }

    pub fn get_words_and_pos(&self, cursor: usize) -> (Vec<(usize, usize)>, CursorPosition) {
        let words = self.get_words();
        let pos = CursorPosition::get(cursor, &words);
        (words, pos)
    }
}
