use readline::editor::Editor;
use readline::history::History;

pub enum Mode {
    Normal,
    Completion,
}

pub struct Context {
    pub editor: Editor,
    pub history: History,
    pub mode: Mode,
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

impl Context {
    pub fn new() -> Self {
        Self {
            editor: Editor::new("> ".into()),
            history: History::new(),
            mode: Mode::Normal,
        }
    }
}
