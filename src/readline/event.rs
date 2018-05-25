use std::io;

use readline::editor::Complete;
use readline::context::Context;

#[derive(Clone, Debug)]
pub enum Kind {
    Enter,
    Delete,
    Complete,
    CtrlL,
    ForwardChar,
    BackwardChar,
    PreviousHistory,
    NextHistory,
    BeginningOFLine,
    EndOfLine,
    // TODO
    Something,
}

pub type Handler = fn(&mut Context, Vec<u8>) -> io::Result<Option<String>>;

pub struct Event {
    // kind: Option<Kind>,
    pub handler: Handler,
}

impl Event {
    pub fn from_event_kind(k: &Option<Kind>) -> Self {
        let h: Handler = match *k {
            Some(Kind::Complete) => {
                |con, _| {
                    if !con.editor.line.trim().len() == con.editor.line.len() {
                        // TODO
                        // editor.put("\t".into())?;
                        return Ok(None);
                    } else {
                        con.editor.complete();
                        // con.editor.completion_prev();
                        con.editor.completion_next();
                        con.mode = true;
                    }
                    Ok(None)
                }
            }
            Some(Kind::Enter) => {
                |con, _| {
                    let result = con.editor.line.clone();
                    con.editor.completion_clear();
                    con.editor.reset();
                    con.editor.new_line();
                    con.history.push(result.clone());
                    con.history.reset();
                    Ok(Some(result))
                }
            }
            Some(Kind::CtrlL) => {
                |con, _| {
                    con.editor.clear_screen();
                    con.editor.write_line();
                    Ok(None)
                }
            }
            Some(Kind::Delete) => {
                |con, _| {
                    con.editor.completion_clear();
                    con.editor.delete(1);
                    Ok(None)
                }
            }
            Some(Kind::ForwardChar) => {
                |con, _| {
                    con.editor.move_right(1);
                    Ok(None)
                }
            }
            Some(Kind::BackwardChar) => {
                |con, _| {
                    con.editor.move_left(1);
                    Ok(None)
                }
            }
            Some(Kind::PreviousHistory) => {
                |Context {
                     editor,
                     history,
                     mode,
                 },
                 _| {
                    if *mode {
                        editor.completion_prev();
                        return Ok(None);
                    }
                    editor.completion_clear();
                    if history.is_started() {
                        history.set_first(editor.line.clone());
                    }
                    let history = match history.prev() {
                        Some(h) => h,
                        None => return Ok(None),
                    };
                    editor.replace(history);
                    editor.move_to_end();
                    Ok(None)
                }
            }
            Some(Kind::NextHistory) => {
                |Context {
                     editor,
                     history,
                     mode,
                 },
                 _| {
                    if *mode {
                        editor.completion_next();
                        return Ok(None);
                    }
                    editor.completion_clear();
                    let history = match history.next() {
                        Some(h) => h,
                        None => return Ok(None),
                    };
                    editor.replace(history);
                    editor.move_to_end();
                    Ok(None)
                }
            }
            Some(Kind::BeginningOFLine) => {
                |con, _| {
                    con.editor.move_to_first();
                    Ok(None)
                }
            }

            Some(Kind::EndOfLine) => {
                |con, _| {
                    con.editor.move_to_end();
                    Ok(None)
                }
            }
            Some(Kind::Something) => |_, _| Ok(None),
            _ => {
                |con, c: Vec<u8>| {
                    con.editor.completion_clear();
                    con.mode = false;

                    // con.editor.complete();
                    // con.editor.completion_next();
                    // con.editor.completion_disply();

                    con.editor.put(&String::from_utf8(c).unwrap());
                    con.history.reset_first();
                    Ok(None)
                }
            }
        };
        Self {
            // kind: k,
            handler: h,
        }
    }
}
