use std::collections::VecDeque;

pub struct History {
    pub list: VecDeque<String>,
    pub pos: usize,
    prev: HistoryExec,
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
        }
    }

    pub fn next(&mut self) -> Option<&String> {
        if self.pos == 0 {
            return None;
        }
        if self.prev == HistoryExec::Prev {
            self.pos -= 1;
            self.prev = HistoryExec::Next;
        }
        self.pos -= 1;
        let ret = self.list.get(self.pos);
        ret
    }

    pub fn prev(&mut self) -> Option<&String> {
        if self.prev == HistoryExec::Next {
            self.pos += 1;
            self.prev = HistoryExec::Prev;
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
}

#[test]
fn test_history() {
    let mut history = History::new();
    history.push("A".to_string());
    history.push("B".to_string());
    history.push("C".to_string());

    assert_eq!(history.next(), Some(&"C".to_string()));
    assert_eq!(history.prev(), Some(&"B".to_string()));
    assert_eq!(history.prev(), Some(&"A".to_string()));
    assert_eq!(history.prev(), None);
    assert_eq!(history.next(), Some(&"B".to_string()));
    assert_eq!(history.next(), Some(&"C".to_string()));
}
