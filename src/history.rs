use std::collections::VecDeque;

pub struct History {
    pub list: VecDeque<String>,
    pub pos: usize,
    prev: HistoryExec,
    first: Option<String>,
}

#[derive(PartialEq)]
enum HistoryExec {
    Next,
    Prev,
}

impl History {
    pub fn new() -> History {
        History {
            list: VecDeque::new(),
            pos: 0,
            prev: HistoryExec::Prev,
            first: None,
        }
    }

    pub fn reset(&mut self) {
        self.pos = 0;
        self.prev = HistoryExec::Prev;
        self.first = None;
    }

    pub fn next(&mut self) -> Option<&String> {
        if self.prev == HistoryExec::Prev {
            self.prev = HistoryExec::Next;
            if self.pos != 0 {
                self.pos -= 1;
            }
        }

        if self.pos == 0 {
            return match self.first {
                Some(ref mut s) => Some(s),
                None => None,
            };
        }
        self.pos -= 1;
        let ret = self.list.get(self.pos);
        ret
    }

    pub fn prev(&mut self) -> Option<&String> {
        if self.prev == HistoryExec::Next {
            self.prev = HistoryExec::Prev;
            if self.pos != 0 {
                self.pos += 1;
            }
        }
        let ret = self.list.get(self.pos);
        if ret.is_none() {
            return None;
        }
        self.pos += 1;
        ret
    }

    pub fn push(&mut self, s: String) {
        self.list.push_front(s);
    }

    pub fn set_first(&mut self, s: String) {
        self.first = Some(s);
    }

    pub fn is_started(&self) -> bool {
        self.pos == 0 && self.first.is_none()
    }

    pub fn reset_first(&mut self) {
        if self.first.is_some() {
            self.first = None;
        }
    }
}

#[test]
fn test_history() {
    let mut history = History::new();
    history.push("A".to_string());
    history.push("B".to_string());
    history.push("C".to_string());
    history.set_first("D".to_string());

    assert_eq!(history.prev(), Some(&"C".to_string()));
    assert_eq!(history.prev(), Some(&"B".to_string()));
    assert_eq!(history.prev(), Some(&"A".to_string()));
    assert_eq!(history.prev(), None);
    assert_eq!(history.next(), Some(&"B".to_string()));
    assert_eq!(history.next(), Some(&"C".to_string()));
    assert_eq!(history.next(), Some(&"D".to_string()));
}
