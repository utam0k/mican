#[derive(Clone, Default)]
pub struct Buffer {
    data: String,
}

impl From<String> for Buffer {
    fn from(s: String) -> Self {
        Self { data: s }
    }
}

impl<'a> From<&'a str> for Buffer {
    fn from(s: &'a str) -> Self {
        Self { data: s.to_owned() }
    }
}

impl Buffer {
    pub fn new() -> Self {
        Self { data: String::new() }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    // TODO
    pub fn as_string(&self) -> &String {
        &self.data
    }

    pub fn remove(&mut self, idx: usize) -> char {
        self.data.remove(idx)
    }

    pub fn insert_str(&mut self, idx: usize, string: &str) {
        self.data.insert_str(idx, string)
    }
}
