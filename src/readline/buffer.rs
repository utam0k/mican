#[derive(Clone, Default)]
pub struct Buffer {
    data: String,
}

impl Buffer {
    pub fn new() -> Self {
        Self { data: String::new() }
    }

    pub fn new_from_str(s: String) -> Self {
        Self { data: s }
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
