use readline::editor::Editor;
use readline::history::History;

pub struct Context {
    pub editor: Editor,
    pub history: History,
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
        }
    }
}
