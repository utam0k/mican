use readline::editor::Editor;
use readline::history::History;
use readline::completer::Completer;

pub enum Mode {
    Normal,
    Completion,
}

pub struct Context {
    pub editor: Editor,
    pub history: History,
    pub mode: Mode,
    pub completer: Box<Completer>,
}

impl Context {
    pub fn new(comp: Box<Completer>) -> Self {
        Self {
            editor: Editor::new("> ".into()),
            history: History::new(),
            mode: Mode::Normal,
            completer: comp,
        }
    }
}
