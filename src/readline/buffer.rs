use std::iter::FromIterator;

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

    pub fn remove(&mut self, idx: usize) -> char {
        self.data.remove(idx)
    }

    pub fn insert_str(&mut self, idx: usize, string: &str) {
        for (i, c) in string.chars().enumerate() {
            self.data.insert(idx + i, c)
        }
    }
}
