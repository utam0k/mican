pub struct History {
    pub list: Vec<String>,
    pub pos: usize,
}

impl History {
    pub fn new() -> History {
        History {
            list: Vec::new(),
            pos: 0,
        }
    }

    pub fn next(&mut self) -> Option<&String> {
        let ret = self.list.get(self.pos);
        if ret.is_some() {
            self.pos -= 1;
        }
        ret
    }

    pub fn prev(&mut self) -> Option<&String> {
        let ret = self.list.get(self.pos);
        if ret.is_some() {
            self.pos += 1;
        }
        ret
    }

    pub fn push(&mut self, s: String) {
        self.list.push(s);
    }

    // pub fn is_start(&self) -> bool {
    //     self.pos == 0
    // }
}
